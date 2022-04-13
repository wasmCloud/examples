//! Rust-Python integration:
//! - initialize python GIL
//! - invoke python methods
//! - auto-reload when changes are detected

use log::{debug, error, info};
use pyo3::Python;
use serde_value::Value;
use std::{
    collections::{BTreeMap, HashMap},
    ffi::OsStr,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::SystemTime,
};
use tokio::sync::RwLock;
use wasmbus_rpc::provider::prelude::*;

mod config;
pub use config::Config;
pub(crate) mod pime;

const THREAD_POOL_MIN_THREADS: u8 = 4;
const THREAD_POOL_MAX_THREADS: u8 = 12;

#[derive(Clone, Debug, Default)]
pub struct InstanceData {
    config: Config,
    modified: Arc<RwLock<Option<SystemTime>>>,
}

#[derive(Clone, Default)]
pub struct Service(pub InstanceData);

impl Service {
    pub async fn try_init(vars: Option<HashMap<String, String>>) -> RpcResult<Self> {
        let vars = vars.unwrap_or_default();

        let config = Config::init(vars)?;
        let service = Service(InstanceData {
            config,
            modified: Arc::new(RwLock::new(None)),
        });

        if !pime::is_engine_started() {
            service
                .init_python()
                .await
                .map_err(|e| RpcError::ProviderInit(e.to_string()))?;
        }
        Ok(service)
    }

    /// stop the instance
    pub async fn shutdown() {
        if let Err(e) = pime::stop().await {
            error!("shutdown error: {}", e);
        }
    }

    async fn init_python(&self) -> Result<(), pime::Error> {
        let py_main = PathBuf::from(&self.0.config.python_main.as_ref().unwrap());
        if py_main.is_dir() {
            if !py_main.join("__init__.py").is_file() {
                return Err(pime::Error::new(
                    pime::ErrorKind::ExecError,
                    &format!(
                        "python_main '{}' is a folder but is not a valid python module (missing \
                         __init__.py)",
                        py_main.display()
                    ),
                ));
            }
        }
        if py_main.is_file() {
            if py_main.extension() != Some(OsStr::new("py")) {
                return Err(pime::Error::new(
                    pime::ErrorKind::ExecError,
                    &format!(
                        "python_main '{}' file does not end in '.py'",
                        py_main.display()
                    ),
                ));
            }
        }
        if let Some(ref py_disp) = self.0.config.python_dispatch {
            let py_disp = PathBuf::from_str(py_disp).unwrap();
            if !(py_disp.is_file() && py_disp.extension() == Some(OsStr::new("py"))) {
                return Err(pime::Error::new(
                    pime::ErrorKind::ExecError,
                    &format!(
                        "invalid python_dispatch '{}': must be readable file ending in '.py'",
                        py_disp.display()
                    ),
                ));
            }
            if let Ok(md) = std::fs::metadata(&py_disp) {
                if let Ok(modified) = md.modified() {
                    let mut m = self.0.modified.write().await;
                    *m = Some(modified);
                }
            }
        }
        // init and start pime
        let config = self.0.config.clone();
        let _join = tokio::task::spawn_blocking(move || {
            // omit if auto-prepared
            pyo3::prepare_freethreaded_python();
            let rc = Python::with_gil(|py| {
                // as Python has GIL,
                // all work with the Python object MUST be performed in this thread
                // after there is no way to reconfigure it
                let engine = match &config.venv_path {
                    Some(venv) => pime::PySyncEngine::new_venv(&py, venv).expect("reading venv"),
                    None => pime::PySyncEngine::new(&py).expect("init engine"),
                };

                // inserts directories into Python's sys.path, starting with the main program dir
                engine
                    .add_import_path(py_main.parent().unwrap().to_str().unwrap())
                    .unwrap();
                for d in config.sys_path.iter() {
                    engine.add_import_path(d).unwrap();
                }
                // enables debug mode
                //engine.enable_debug().unwrap();
                engine.set_thread_pool_size(
                    config.min_threads.unwrap_or(THREAD_POOL_MIN_THREADS) as u32,
                    config.max_threads.unwrap_or(THREAD_POOL_MAX_THREADS) as u32,
                )?;
                let py_main_module = if py_main.is_file() {
                    py_main.file_stem().unwrap().to_str().unwrap()
                } else {
                    py_main.file_name().unwrap().to_str().unwrap()
                };
                eprintln!("about to load python main module {}", py_main_module);
                let module = py.import(py_main_module).unwrap();
                let broker = module.getattr("main").unwrap();
                // Perform additional work, e.g. add Rust functions to Python modules
                // .................................
                // fire and go
                engine.launch(&py, broker).unwrap();
                Ok::<(), pime::Error>(())
            });
            if let Err(e) = rc {
                error!("error starting python: {}", e);
            }
        });
        // wait engine to be started
        pime::wait_online().await;
        Ok(())
    }

    /// check last modified date of dispatch module to see if it's changed.
    /// If so, shut down engine and reload
    async fn reload_needed(&self) -> bool {
        // modified date of file at time of last load
        let previous_modified = self.0.modified.read().await.clone();
        // modified date of file now
        let current_modified = {
            let py_disp = PathBuf::from(&self.0.config.python_dispatch.as_ref().unwrap());
            if let Ok(md) = std::fs::metadata(&py_disp) {
                if let Ok(modified) = md.modified() {
                    Some(modified)
                } else {
                    None
                }
            } else {
                None
            }
        };
        match (previous_modified, current_modified) {
            (None, Some(_)) => true,
            (Some(l), Some(d)) if d > l => true,
            _ => false,
        }
    }

    // invoke, checking whether dispatch reload is required
    pub async fn check_invoke(&self, command: &str, arg: &[u8]) -> Result<Vec<u8>, RpcError> {
        if self.reload_needed().await {
            info!("dispatch change detected, reloading...");
            let _ = self.invoke("reload", &[]).await?;
        }
        self.invoke(command, arg).await
    }

    /// Invoke python with command and arg
    /// Deserialize param into value for python, serialize result to return to caller
    pub async fn invoke(&self, command: &str, arg: &[u8]) -> Result<Vec<u8>, RpcError> {
        let mut params = BTreeMap::new();
        if !arg.is_empty() {
            let value: Value =
                minicbor_ser::from_slice(arg).map_err(|e| RpcError::Ser(e.to_string()))?;
            debug!("Invoking {}(arg: {:?})", command, &value);
            params.insert("arg".to_string(), value);
        } else {
            debug!("Invoking {}()", command);
        }
        let mut task = pime::PyTask::new(Value::String(command.to_string()), params);
        // If the task result is not required, the task can be marked to be executed
        // forever in ThreadPoolExecutor, until finished. In this case, "call" always
        // returns result None
        //task.no_wait();
        // If a task performs calculations only, it can be marked as exclusive.
        // Tasks of this type lock Python thread until completed. Use with care!
        task.mark_exclusive();
        match pime::call(task).await {
            Ok(result) => {
                debug!("Result: {:?}", &result);
                let buf = match result {
                    Some(r) => {
                        minicbor_ser::to_vec(&r).map_err(|e| RpcError::Deser(e.to_string()))?
                    }
                    None => Vec::new(),
                };
                Ok(buf)
            }
            Err(e) if e.kind == pime::ErrorKind::PyException => {
                let mut error = format!("Exception raised {}: {}", e.exception.unwrap(), e.message);
                error.push_str(&format!("\n{}\n", e.traceback.unwrap()));
                error!("{}", &error);
                Err(RpcError::Other(error))
            }
            Err(e) => {
                let error = format!("error: {}", e.message);
                error!("{}", &error);
                Err(RpcError::Other(error))
            }
        }
    }
}
