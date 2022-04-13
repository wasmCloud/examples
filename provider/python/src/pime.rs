//! # Rust Python Integration Made Easy (from [pime crate](https://crates.io/crates/pime) )
//!
//! ## What is this for
//!
//! PIME is a Rust crate, which allows [tokio](https://tokio.rs)-based Rust
//! programs to easily execute Python code snippets.
//!
//! PIME is based on [PyO3](https://pyo3.rs/), [Serde](https://serde.rs) and
//! [neotasker](https://pypi.org/project/neotasker/) Python module.
//!
//! PIME allows Rust to execute Python blocking code in parallel, using standard
//! ThreadPoolExecutor and await for results of called concurrent.futures objects.
//!
//! PIME is absolutely thread-safe and has got a goal to make Python integration
//! into Rust as simple as possible.
//!
//! PIME allows Rust programs to have Python-based extensions, plugins, integration
//! of Python scripts directly into Rust web servers etc.
//!
//! ## How does it work
//!
//! Let us look inside PIME-integrated Rust program:
//!
//! |Rust thread | Rust thread | Rust thread | Python GIL thread  |
//! |------------|-------------|-------------|--------------------|
//! |rust code   | rust code   | rust code   | ThreadPoolExecutor |
//! |rust code   | await task1 | rust code   | task1              |
//! |await task2 | await task3 | rust code   | task2 task3        |
//!
//! When a Rust coroutine wants to execute a Python task, it creates
//! **pime::PyTask** object and executes **pime::call** method. If the execution is
//! successful, the result is returned as
//! [serde-value::Value](https://crates.io/crates/serde-value) object, otherwise as
//! **pime::Error**, which contains either Python exception information or an
//! engine error.
//!
//! On the Python side, all tasks are handled by the broker function. The function
//! has two arguments: *command* and *params* (no keyword-based arguments, sorry -
//! current limitation of the asyncio loops' *run\_in\_executor* function). Broker
//! function instances are launched in parallel, using Python's ThreadPoolExecutor.
//!
//! When the broker returns a result or raises an exception, this is reported back
//! to the Rust code.
//!
//! Communication is performed via thread-safe mpsc channels.
//!
//! ## Usage example
//!
//! ### Preparing
//!
//! Install neotasker module for Python:
//!
//! ```shell
//! pip3 install neotasker
//! ```
//!
//! ### Cargo dependencies
//!
//! ```toml
//! [dependencies]
//! tokio = { version = "1.4", features = ["full"] }
//! pyo3 = { version = "0.14.1" }
//! serde-value = "0.7.0"
//! pime = "*"
//! ```
//!
//! ### Rust code
//!
//! ```rust,ignore
//! use pyo3::prelude::*;
//! use serde_value::Value;
//! use std::collections::BTreeMap;
//! use std::env;
//!
//! // create tokio runtime or use #[tokio::main]
//! // ...............................................
//! // ...............................................
//!
//!
//! // init and start PIME
//! tokio::task::spawn_blocking(move || {
//!     // omit if auto-prepared
//!     pyo3::prepare_freethreaded_python();
//!     Python::with_gil(|py| {
//!         // as Python has GIL,
//!         // all work with the Python object MUST be performed in this thread
//!         // after there is no way to reconfigure it
//!         let engine = pime::PySyncEngine::new(&py).unwrap();
//!         // inserts directories into Python's sys.path
//!         let cwd = env::current_dir().unwrap().to_str().unwrap().to_owned();
//!         engine.add_import_path(&cwd).unwrap();
//!         // enables debug mode
//!         engine.enable_debug().unwrap();
//!         // sets ThreadPoolExecutor size to min = 10, max = 10
//!         engine.set_thread_pool_size(10, 10).unwrap();
//!         let module = py.import("mymod").unwrap();
//!         let broker = module.getattr("broker").unwrap();
//!         // Perform additional work, e.g. add Rust functions to Python modules
//!         // .................................
//!         // fire and go
//!         engine.launch(&py, broker).unwrap();
//!     });
//! });
//! // wait engine to be started
//! pime::wait_online().await;
//!
//! // Done! Now tasks can be called from any coroutine
//! // ...............................................
//! // ...............................................
//!
//! let mut params = BTreeMap::new();
//! params.insert("name".to_owned(), Value::String("Rust".to_owned()));
//! let mut task = pime::PyTask::new(Value::String("hello".to_owned()), params);
//! // If the task result is not required, the task can be marked to be executed
//! // forever in ThreadPoolExecutor, until finished. In this case, "call" always
//! // returns result None
//! //task.no_wait();
//! // If a task performs calculations only, it can be marked as exclusive.
//! // Tasks of this type lock Python thread until completed. Use with care!
//! //task.mark_exclusive();
//! match pime::call(task).await {
//!     Ok(result) => {
//!         // The result is returned as Option<Value>
//!         println!("{:?}", result);
//!     },
//!     Err(e) if e.kind == pime::ErrorKind::PyException => {
//!         println!("Exception raised {}: {}", e.exception.unwrap(), e.message);
//!         println!("{}", e.traceback.unwrap());
//!     }
//!     Err(e) => {
//!         println!("An error is occurred: {}", e.message);
//!     }
//! };
//! // stop the engine gracefully
//! pime::stop().await;
//! ```
//!
//! ### Python code (mymod/\_\_init\_\_.py)
//!
//! ```python
//! def broker(command, params):
//!     if command == 'hello':
//!         return f'Hi from Python, {params["name"]}!'
//!     elif command == 'bye':
//!         return 'Bye bye'
//!     else:
//!         raise RuntimeError('command unsupported')
//! ```
//!
//! ## More examples
//!
//! <https://github.com/alttch/pime/tree/main/examples/>
//!
#![allow(dead_code)]

use lazy_static::lazy_static;
use log::{debug, error};
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use rust_ini::Ini;
use serde_value::Value;
use std::{
    collections::{btree_map, BTreeMap},
    fmt,
    sync::atomic::{AtomicU8, Ordering},
    time::Duration,
};
use tokio::{
    sync::{mpsc, Mutex, RwLock},
    time::sleep,
};

pub const STATE_STOPPED: u8 = 0;
//pub const STATE_STARTING: u8 = 1;
pub const STATE_STOPPING: u8 = 2;
pub const STATE_STARTED: u8 = 0xff;

const DATACHANNEL_DEFAULT_BUFFER: usize = 1024;

const PIME_POLL_DELAY: Duration = Duration::from_millis(1);

static ENGINE_STATE: AtomicU8 = AtomicU8::new(STATE_STOPPED);

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    PyException,
    PackError,
    UnpackError,
    ExecError,
    InternalError,
    PySyncEngineStateError,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorKind::PyException => "Python exception",
                ErrorKind::PackError => "Data pack error",
                ErrorKind::UnpackError => "Data unpack error",
                ErrorKind::ExecError => "Task execution error",
                ErrorKind::InternalError => "Internal error",
                ErrorKind::PySyncEngineStateError => "Engine state error",
            }
        )
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub exception: Option<String>,
    pub traceback: Option<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.kind == ErrorKind::PyException {
            let mut exc = "Python exception".to_owned();
            if let Some(exception) = self.exception.as_ref() {
                exc += " ";
                exc += exception;
                if !self.message.is_empty() {
                    exc += ":";
                }
            };
            write!(f, "{} {}", exc, self.message)
        } else {
            write!(f, "{}: {}", self.kind, self.message)
        }
    }
}

impl From<PyErr> for Error {
    fn from(e: PyErr) -> Error {
        Error::new_internal(e)
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error
where
    T: std::fmt::Debug,
{
    fn from(e: tokio::sync::mpsc::error::SendError<T>) -> Error {
        Error::new_internal(e)
    }
}

impl From<tokio::sync::TryLockError> for Error {
    fn from(e: tokio::sync::TryLockError) -> Error {
        Error::new_internal(e)
    }
}

impl Error {
    pub fn new<T: fmt::Display>(kind: ErrorKind, message: T) -> Self {
        Self {
            kind,
            message: format!("{}", message),
            exception: None,
            traceback: None,
        }
    }

    fn new_py(error: (String, String, String)) -> Self {
        Self {
            kind: ErrorKind::PyException,
            exception: Some(error.0),
            message: error.1,
            traceback: Some(error.2),
        }
    }

    fn new_internal<T: fmt::Display>(message: T) -> Self {
        Self {
            kind: ErrorKind::PySyncEngineStateError,
            message: format!("CRITICAL: PySyncEngine internal error: {}", message),
            exception: None,
            traceback: None,
        }
    }

    fn new_offline() -> Self {
        Self {
            kind: ErrorKind::PySyncEngineStateError,
            message: "PySyncEngine is offline".to_owned(),
            exception: None,
            traceback: None,
        }
    }

    fn new_online() -> Self {
        Self {
            kind: ErrorKind::PySyncEngineStateError,
            message: "PySyncEngine is online".to_owned(),
            exception: None,
            traceback: None,
        }
    }
}

#[derive(Debug)]
pub struct PyTask {
    command: Value,
    params: BTreeMap<String, Value>,
    need_result: bool,
    exclusive: bool,
}

#[allow(dead_code)]
impl PyTask {
    #[must_use]
    pub fn new(command: Value, params: BTreeMap<String, Value>) -> Self {
        Self {
            command,
            params,
            need_result: true,
            exclusive: false,
        }
    }

    #[must_use]
    pub fn new0(command: Value) -> Self {
        Self {
            command,
            params: BTreeMap::new(),
            need_result: true,
            exclusive: false,
        }
    }

    /*
    pub fn no_wait(&mut self) {
        self.need_result = false;
    }
     */

    pub fn mark_exclusive(&mut self) {
        self.exclusive = true;
        self.need_result = true;
    }
}

struct DataChannel {
    tx: Mutex<mpsc::Sender<(u64, Option<PyTask>)>>,
    rx: Mutex<mpsc::Receiver<(u64, Option<PyTask>)>>,
}

impl DataChannel {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<(u64, Option<PyTask>)>(DATACHANNEL_DEFAULT_BUFFER);
        Self {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
        }
    }

    #[allow(dead_code)]
    fn set_buffer(&mut self, buffer: usize) {
        let (tx, rx) = mpsc::channel::<(u64, Option<PyTask>)>(buffer);
        self.tx = Mutex::new(tx);
        self.rx = Mutex::new(rx);
    }
}

#[derive(Debug)]
struct PyTaskResult {
    #[allow(dead_code)]
    task_id: u64,
    ready: triggered::Trigger,
    result: Option<Value>,
    error: Option<Error>,
}

impl PyTaskResult {
    #[allow(clippy::redundant_closure)]
    fn set_result(&mut self, result: Option<Value>) {
        self.result = result;
    }

    fn set_error(&mut self, error: Error) {
        self.error = Some(error);
    }
}

struct PyTaskCounter {
    id: u64,
}

impl PyTaskCounter {
    fn new() -> Self {
        Self { id: 0 }
    }

    fn get(&mut self) -> u64 {
        if self.id == std::u64::MAX {
            self.id = 1;
        } else {
            self.id += 1;
        }
        self.id
    }
}

lazy_static! {
    static ref PY_RESULTS: RwLock<BTreeMap<u64, PyTaskResult>> = RwLock::new(BTreeMap::new());
    static ref TASK_COUNTER: Mutex<PyTaskCounter> = Mutex::new(PyTaskCounter::new());
    static ref DC: RwLock<DataChannel> = RwLock::new(DataChannel::new());
}

pub struct PySyncEngine<'p> {
    neo: &'p pyo3::types::PyModule,
}

macro_rules! need_online {
    () => {
        if ENGINE_STATE.load(Ordering::SeqCst) != STATE_STARTED {
            return Err(Error::new_offline());
        }
    };
}

macro_rules! need_offline {
    () => {
        if ENGINE_STATE.load(Ordering::SeqCst) != STATE_STOPPED {
            return Err(Error::new_online());
        }
    };
}

macro_rules! critical {
    ($msg: expr) => {
        error!("PySyncEngine CRIICAL: {}", $msg);
    };
}

macro_rules! log_lost_task {
    ($task_id: expr) => {
        error!("PySyncEngine CRIICAL: task {} is lost", $task_id);
    };
}

fn report_error(task_id: u64, error: Error) {
    loop {
        if let Ok(mut v) = PY_RESULTS.try_write() {
            if let Some(o) = v.get_mut(&task_id) {
                o.set_error(error);
                o.ready.trigger();
                break;
            }
            log_lost_task!(task_id);
            return;
        }
        std::thread::sleep(PIME_POLL_DELAY);
        continue;
    }
}

#[pyfunction]
fn report_result(
    py: Python,
    task_id: u64,
    result: Option<Py<PyAny>>,
    error: Option<(String, String, String)>,
) {
    let data: Option<Value> = if let Some(r) = result {
        match depythonize(r.as_ref(py)) {
            Ok(v) => v,
            Err(e) => {
                report_error(task_id, Error::new(ErrorKind::UnpackError, e));
                return;
            }
        }
    } else {
        None
    };
    loop {
        if let Ok(mut v) = PY_RESULTS.try_write() {
            if let Some(o) = v.get_mut(&task_id) {
                o.set_result(data);
                if let Some(e) = error {
                    o.set_error(Error::new_py(e));
                }
                o.ready.trigger();
                break;
            }
            log_lost_task!(task_id);
            return;
        }
        std::thread::sleep(PIME_POLL_DELAY);
        continue;
    }
}

impl<'p> PySyncEngine<'p> {
    /// # Errors
    ///
    /// Will return Err if the Python engine is failed to initialize itself
    pub fn new(py: &'p pyo3::Python) -> Result<Self, Error> {
        PySyncEngine::new_engine(py, None)
    }

    /// # Errors
    ///
    /// Will return Err if the Python engine is failed to initialize itself or venv is
    /// broken/invalid
    pub fn new_venv(py: &'p pyo3::Python, venv_path: &str) -> Result<Self, Error> {
        PySyncEngine::new_engine(py, Some(venv_path))
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[allow(clippy::too_many_lines)]
    fn new_engine(py: &'p pyo3::Python, venv_path: Option<&str>) -> Result<Self, Error> {
        if ENGINE_STATE.load(Ordering::SeqCst) != STATE_STOPPED {
            return Err(Error::new_online());
        }
        if let Some(dir) = venv_path {
            let cfg = format!("{}/pyvenv.cfg", dir);
            let ini = match Ini::load_from_file(&cfg) {
                Ok(v) => v,
                Err(e) => {
                    return Err(Error::new(
                        ErrorKind::InternalError,
                        format!("Unable to read venv config file {}: {}", cfg, e),
                    ));
                }
            };
            let ver_info = py.version_info();
            macro_rules! unwrap_ver_err {
                ($v: expr) => {
                    match $v {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(Error::new(
                                ErrorKind::PyException,
                                format!("Unable to parse venv version info: {}", e),
                            ));
                        }
                    }
                };
            }
            macro_rules! unwrap_ver {
                ($v: expr) => {
                    if let Some(v) = $v {
                        v
                    } else {
                        return Err(Error::new(
                            ErrorKind::PyException,
                            "Unable to get venv version info".to_owned(),
                        ));
                    }
                };
            }
            debug!("DBG: ver_info: {:?}", &ver_info);
            let venv_ver = if let Some(vv) = ini.general_section().get("version") {
                unwrap_ver!(Some(vv))
            } else {
                unwrap_ver!(ini.general_section().get("version_info"))
            };
            debug!("DBG: venv version: {}", venv_ver);
            let mut s = venv_ver.split('.');
            let venv_major = unwrap_ver_err!(unwrap_ver!(s.next()).parse::<u8>());
            let venv_minor = unwrap_ver_err!(unwrap_ver!(s.next()).parse::<u8>());
            if venv_major != ver_info.major || venv_minor != ver_info.minor {
                return Err(Error::new(
                    ErrorKind::PyException,
                    format!(
                        "Unable to activate venv, Python library version: {}.{}, venv version: \
                         {}. Please switch the library or rebuild venv",
                        ver_info.major, ver_info.minor, venv_ver
                    ),
                ));
            }
            if let Some(v) = ini.general_section().get("include-system-site-packages") {
                if v == "false" {
                    debug!("Removing system-site packages from Python path");
                    py.run(
                        "import sys;list(map(lambda x:sys.path.remove(x) if \
                         x.endswith('-packages') or '/dist-packages/' in x or '/site-packages/' \
                         in x else False, sys.path.copy()))",
                        None,
                        None,
                    )?;
                }
            }
            let import_path = format!(
                "{}/lib/python{}.{}/site-packages",
                dir, ver_info.major, ver_info.minor
            );
            debug!("Adding Python venv import path: {}", import_path);
            py.run(
                &format!("import sys;sys.path.insert(0,'{}')", import_path),
                None,
                None,
            )?;
            //py.run(
            //"import sys;print(sys.path)",
            //None,
            //None,
            //)?;
        }
        let neo = py.import("neotasker.embed")?;
        neo.add_function(wrap_pyfunction!(report_result, neo)?)?;
        Ok(Self { neo })
    }

    /// # Errors
    ///
    /// Will return Err if the Python neotasker module failed to add the import path
    pub fn add_import_path(&self, dir: &str) -> Result<(), Error> {
        self.neo.call_method1("add_import_path", (dir,))?;
        Ok(())
    }

    /// # Errors
    ///
    /// Will return Err if the Python neotasker module failed to enable debugging
    pub fn enable_debug(&self) -> Result<(), Error> {
        self.neo.call_method0("set_debug")?;
        Ok(())
    }

    /*
    /// # Errors
    ///
    /// Will return Err if the Python neotasker module failed to set the poll delay
    pub fn set_poll_delay(&self, delay: f32) -> Result<(), Error> {
        self.neo.call_method1("set_poll_delay", (delay,))?;
        Ok(())
    }
     */

    /// # Errors
    ///
    /// Will return Err if the Python neotasker module failed to set the thread pool size
    pub fn set_thread_pool_size(&self, min: u32, max: u32) -> Result<(), Error> {
        self.neo
            .call_method1("set_thread_pool_size", ((min, max),))?;
        Ok(())
    }

    /// # Errors
    ///
    /// Will return Err if the engine is already launched or Python neotasker module is failed to
    /// be intialized
    pub fn launch(&self, py: &'p pyo3::Python, broker: &pyo3::PyAny) -> Result<(), Error> {
        need_offline!();

        let dc = DC.try_read()?;
        let mut rx = dc.rx.try_lock()?;

        self.neo.call_method0("start")?;
        let call = self.neo.getattr("call")?;
        let call_direct = self.neo.getattr("call_direct")?;
        let spawn = self.neo.getattr("spawn")?;

        ENGINE_STATE.store(STATE_STARTED, Ordering::SeqCst);
        debug!("PySyncEngine started");
        loop {
            if let Some((task_id, t)) = py.allow_threads(|| rx.blocking_recv()) {
                if let Some(task) = t {
                    let command = match pythonize(*py, &task.command) {
                        Ok(v) => v,
                        Err(e) => {
                            if task_id != 0 {
                                report_error(task_id, Error::new(ErrorKind::PackError, e));
                            };
                            continue;
                        }
                    };
                    let params = match pythonize(*py, &task.params) {
                        Ok(v) => v,
                        Err(e) => {
                            if task_id != 0 {
                                report_error(task_id, Error::new(ErrorKind::PackError, e));
                            };
                            continue;
                        }
                    };
                    if task.exclusive {
                        if let Err(e) = call_direct.call1((task_id, broker, command, params)) {
                            report_error(task_id, Error::new(ErrorKind::ExecError, e));
                        }
                    } else if task.need_result {
                        if let Err(e) = call.call1((task_id, broker, command, params)) {
                            report_error(task_id, Error::new(ErrorKind::ExecError, e));
                        }
                    } else {
                        let _r = spawn.call1((broker, command, params));
                    }
                } else {
                    ENGINE_STATE.store(STATE_STOPPING, Ordering::SeqCst);
                    break;
                }
            } else {
                return Err(Error::new_internal("channel broken".to_owned()));
            }
        }
        debug!("Stopping PySyncEngine");
        self.neo.call_method0("stop")?;
        debug!("PySyncEngine stopped");
        ENGINE_STATE.store(STATE_STOPPED, Ordering::SeqCst);
        Ok(())
    }
}

/// # Errors
///
/// Will return Err if the engine is stopped or broken
pub async fn call(task: PyTask) -> Result<Option<Value>, Error> {
    need_online!();
    if !task.need_result {
        DC.read()
            .await
            .tx
            .lock()
            .await
            .send((0, Some(task)))
            .await?;
        return Ok(None);
    }
    let (trigger, listener) = triggered::trigger();
    let task_id = loop {
        let cid = TASK_COUNTER.lock().await.get();
        if let btree_map::Entry::Vacant(x) = PY_RESULTS.write().await.entry(cid) {
            x.insert(PyTaskResult {
                task_id: cid,
                result: None,
                error: None,
                ready: trigger,
            });
            break cid;
        }
        critical!("dead tasks in result map");
    };
    DC.read()
        .await
        .tx
        .lock()
        .await
        .send((task_id, Some(task)))
        .await?;
    listener.await;
    PY_RESULTS.write().await.remove(&task_id).map_or_else(
        || {
            Err(Error::new(
                ErrorKind::InternalError,
                "CRITICAL: Result not found, engine broken".to_owned(),
            ))
        },
        |res| res.error.map_or(Ok(res.result), Err),
    )
}

/// # Errors
///
/// Will return Err if the engine is already stopped
pub async fn stop() -> Result<(), Error> {
    need_online!();
    DC.read().await.tx.lock().await.send((0, None)).await?;
    wait_offline().await;
    Ok(())
}

/*
pub fn get_engine_state() -> u8 {
    ENGINE_STATE.load(Ordering::SeqCst)
}
 */

#[must_use]
pub fn is_engine_started() -> bool {
    ENGINE_STATE.load(Ordering::SeqCst) == STATE_STARTED
}

pub async fn wait_online() {
    while ENGINE_STATE.load(Ordering::SeqCst) != STATE_STARTED {
        sleep(PIME_POLL_DELAY).await;
    }
}

pub async fn wait_offline() {
    while ENGINE_STATE.load(Ordering::SeqCst) != STATE_STOPPED {
        sleep(PIME_POLL_DELAY).await;
    }
}

/*
/// # Panics
///
/// Will panic if the engine is on and the data channel is busy
pub fn set_mpsc_buffer(buffer: usize) {
    DC.try_write().unwrap().set_buffer(buffer);
}
 */
