//! Configuration for python capability provider
//!
//!
use log::debug;
use serde::Deserialize;
use std::{collections::HashMap, env, path::PathBuf};
use wasmbus_rpc::error::{RpcError, RpcResult};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    /// path to main program to run: env PYTHON_MAIN
    pub python_main: Option<String>,

    /// path to main program to run: env PYTHON_DISPATCH
    pub python_dispatch: Option<String>,

    /// optional venv folder: env VENV_PATH
    pub venv_path: Option<String>,

    /// sys.path - where to load other libraries: venv PYTHON_PATH
    pub sys_path: Vec<String>,

    /// max size of thread pool
    pub max_threads: Option<u8>,

    /// min/initial size of thread pool
    pub min_threads: Option<u8>,
}

impl Config {
    /// initialize from linkdef values, override from environment
    pub fn init(values: HashMap<String, String>) -> RpcResult<Config> {
        let mut config = if let Some(config_b64) = values.get("config_b64") {
            let bytes = base64::decode(config_b64.as_bytes()).map_err(|e| {
                RpcError::InvalidParameter(format!("invalid base64 encoding: {}", e))
            })?;
            serde_json::from_slice::<Config>(&bytes)
                .map_err(|e| RpcError::InvalidParameter(format!("corrupt config_b64: {}", e)))?
        } else if let Some(config) = values.get("config_json") {
            serde_json::from_str::<Config>(config)
                .map_err(|e| RpcError::InvalidParameter(format!("corrupt config_json: {}", e)))?
        } else {
            Config::default()
        };
        if let Ok(prog) = env::var("PYTHON_MAIN") {
            debug!("using python_main: {}", &prog);
            config.python_main = Some(prog);
        }
        if let Ok(prog) = env::var("PYTHON_DISPATCH") {
            debug!("using python_dispatch: {}", &prog);
            config.python_dispatch = Some(prog);
        }
        if let Ok(syspath) = env::var("PYTHON_PATH") {
            if !syspath.is_empty() {
                for part in syspath.split(':') {
                    debug!("adding python dir: {}", part);
                    config.sys_path.push(part.to_string());
                }
            }
        }
        if let Ok(venv) = env::var("VENV_PATH") {
            debug!("using venv: {}", &venv);
            config.venv_path = Some(venv);
        }
        if let Some(venv) = &config.venv_path {
            if !PathBuf::from(&venv).is_dir() {
                return Err(RpcError::ProviderInit(format!(
                    "venv_path '{}' is not a valid folder",
                    &venv
                )));
            }
        }
        if let Some(pp) = &config.python_main {
            if !PathBuf::from(&pp).exists() {
                return Err(RpcError::ProviderInit(format!(
                    "python_main '{}' is not a valid file or directory",
                    &pp
                )));
            }
        } else {
            return Err(RpcError::ProviderInit(
                "missing config setting for 'python_main': path to startup module".to_string(),
            ));
        }
        if let Some(pp) = &config.python_dispatch {
            if !PathBuf::from(&pp).is_file() {
                return Err(RpcError::ProviderInit(format!(
                    "python_dispatch '{}' is not a valid file",
                    &pp
                )));
            }
        }
        debug!("Config: {:?}", &config);
        Ok(config)
    }
}
