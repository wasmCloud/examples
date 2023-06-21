use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;

use tempfile::TempDir;
use tokio::net::TcpListener;
use tokio::process::{Child, Command};

const HTTP_SERVER_PROVIDER_ID: &str = "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M";
const BLOBSTORE_PROVIDER_ID: &str = "VBBQNNCGUKIXEWLL5HL5XJE57BS3GU5DMDOKZS6ROEWPQFHEDP6NGVZM";
const DEFAULT_WASMCLOUD_PORT: u16 = 4000;
const DEFAULT_NATS_PORT: u16 = 4222;

// NOTE: this wash setup code is wholesale copied from wadm. We should probably put this in a library

/// Get a TCP random port
async fn get_random_tcp_port() -> u16 {
    TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
        .await
        .expect("Unable to bind to check for port")
        .local_addr()
        .unwrap()
        .port()
}

#[derive(Debug)]
pub struct CleanupGuard {
    child: Option<Child>,
    _log_dir: TempDir,
    stdout_file: PathBuf,
    stderr_file: PathBuf,
}

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.start_kill();
        }
    }
}

impl CleanupGuard {
    /// Reads all of stdout from the child process and returns it
    async fn stdout(&self) -> Vec<u8> {
        tokio::fs::read(&self.stdout_file)
            .await
            .expect("Unable to read stdout")
    }

    /// Reads all of stderr from the child process and returns it
    #[allow(dead_code)]
    async fn stderr(&self) -> Vec<u8> {
        tokio::fs::read(&self.stderr_file)
            .await
            .expect("Unable to read stderr")
    }
}

/// Configuration struct for wash instances that are used for testing
#[derive(Debug, Default)]
struct TestWashConfig {
    /// Port on which to run wasmCloud
    nats_port: Option<u16>,

    /// Only connect to pre-existing NATS instance
    nats_connect_only: bool,

    /// Port on which to run wasmCloud (via `wash up`)
    wasmcloud_port: Option<u16>,
}

impl TestWashConfig {
    /// Build a test wash configuration with randomized ports
    async fn random() -> TestWashConfig {
        let nats_port = Some(get_random_tcp_port().await);
        let wasmcloud_port = Some(get_random_tcp_port().await);

        TestWashConfig {
            nats_port,
            wasmcloud_port,
            ..TestWashConfig::default()
        }
    }

    /// Get the washboard URL for this config
    fn washboard_url(&self) -> String {
        format!(
            "localhost:{}",
            self.wasmcloud_port.unwrap_or(DEFAULT_WASMCLOUD_PORT)
        )
    }
}

/// Start a local wash instance
async fn start_wash_instance(cfg: &TestWashConfig) -> CleanupGuard {
    let nats_port = cfg.nats_port.unwrap_or(DEFAULT_NATS_PORT).to_string();
    let wasmcloud_port = cfg
        .wasmcloud_port
        .unwrap_or(DEFAULT_WASMCLOUD_PORT)
        .to_string();

    // Build args
    let mut args: Vec<&str> = Vec::from(["up", "--nats-port", &nats_port]);
    if cfg.nats_connect_only {
        args.push("--nats-connect-only");
    }

    // NOTE(thomastaylor312): I tried to do this by reading from piped stdin/out but `read_to_end`
    // didn't work because it was waiting for EOF. I didn't want to waste time writing my own read
    // that would check when it got pending and then return, so I did this instead
    let log_dir = tempfile::tempdir().expect("Unable to create tempdir");
    let stdout_file = log_dir.as_ref().join("stdout");
    let stderr_file = log_dir.as_ref().join("stderr");
    let stdout = tokio::fs::File::create(&stdout_file)
        .await
        .expect("Unable to create stdout file");
    let stderr = tokio::fs::File::create(&stderr_file)
        .await
        .expect("Unable to create stderr file");

    // Build the command
    let mut cmd = Command::new("wash");
    let child = cmd
        .args(&args)
        .env("WASMCLOUD_PORT", &wasmcloud_port)
        .env("WASMCLOUD_DASHBOARD_PORT", &wasmcloud_port)
        .stderr(stderr.into_std().await)
        .stdout(stdout.into_std().await)
        .kill_on_drop(true)
        .spawn()
        .expect("Unable to run wash up");

    let guard = CleanupGuard {
        child: Some(child),
        _log_dir: log_dir,
        stdout_file,
        stderr_file,
    };
    // Make sure we can connect to washboard
    wait_for_server(&cfg.washboard_url(), &guard).await;

    // Give the host just a bit more time to get totally ready
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    guard
}

/// Set up and run a wash instance that can be used for a test
async fn setup_test_wash(cfg: &TestWashConfig) -> CleanupGuard {
    match tokio::net::TcpStream::connect(cfg.washboard_url()).await {
        Err(_) => start_wash_instance(cfg).await,
        Ok(_) => CleanupGuard {
            child: None,
            _log_dir: tempfile::tempdir().unwrap(),
            stdout_file: PathBuf::new(),
            stderr_file: PathBuf::new(),
        },
    }
}

async fn wait_for_server(url: &str, guard: &CleanupGuard) {
    let mut wait_count = 1;
    loop {
        // Magic number: 10 + 1, since we are starting at 1 for humans
        if wait_count >= 11 {
            let out = guard.stdout().await;
            let stdout = String::from_utf8_lossy(&out);
            let err = guard.stderr().await;
            let stderr = String::from_utf8_lossy(&err);
            panic!("Ran out of retries waiting for host to start. Server logs:\nStdout:\n{stdout}\nStderr:\n{stderr}");
        }
        match tokio::net::TcpStream::connect(url).await {
            Ok(_) => break,
            Err(e) => {
                eprintln!("Waiting for server {url} to come up, attempt {wait_count}. Will retry in 1 second. Got error {e:?}");
                wait_count += 1;
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
}

/// Runs wash with the given args and makes sure it runs successfully. Returns the contents of
/// stdout
pub async fn run_wash_command<I, S>(args: I) -> Vec<u8>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let output = Command::new("wash")
        .args(args)
        .output()
        .await
        .expect("Unable to run wash command");
    if !output.status.success() {
        panic!(
            "wash command didn't exit successfully: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
    output.stdout
}

// TODO: Make this actually be unique for each test so we can run in parallel
pub async fn setup_test() -> (String, CleanupGuard) {
    // Build module
    let output = Command::new("wash")
        .args(["build", "-o", "json"])
        .current_dir("../actor")
        .output()
        .await
        .expect("Unable to run wash build");
    assert!(output.status.success(), "Error trying to build module");

    let build_output = serde_json::from_slice::<serde_json::Value>(&output.stdout)
        .expect("Unable to decode build output");

    let actor_path = build_output
        .get("actor_path")
        .and_then(|v| v.as_str())
        .expect("Couldn't find actor path");

    // Get the actor ID
    let output = run_wash_command(["claims", "inspect", "-o", "json", actor_path]).await;

    let claims =
        serde_json::from_slice::<serde_json::Value>(&output).expect("Unable to decode claims");
    let id = claims
        .get("module")
        .and_then(|v| v.as_str())
        .expect("Couldn't find module ID");

    // NOTE (thomastaylor312): I decided just to use wash to start everything since it does the
    // waiting for me and it mimics what a user would do IRL

    // Start wasmcloud host if we don't find one running
    let wash_cfg = TestWashConfig::random().await;
    let guard = setup_test_wash(&wash_cfg).await;

    // TODO: Be idempotent starting providers

    let connection_port = wash_cfg.nats_port.as_ref().unwrap().to_string();
    // Start http server
    let output = Command::new("wash")
        .args([
            "start",
            "provider",
            "-p",
            &connection_port,
            "wasmcloud.azurecr.io/httpserver:0.18.2",
        ])
        .status()
        .await
        .expect("Unable to start http provider");
    assert!(output.success(), "Error trying to start http provider",);

    // Start blobstore_fs
    let output = Command::new("wash")
        .args([
            "start",
            "provider",
            "-p",
            &connection_port,
            "wasmcloud.azurecr.io/blobstore_fs:0.3.2",
        ])
        .status()
        .await
        .expect("Unable to start blobstore provider");
    assert!(output.success(), "Error trying to start blobstore provider",);

    // Start module
    let output = Command::new("wash")
        .args([
            "start",
            "actor",
            "-p",
            &connection_port,
            &format!("file://{actor_path}"),
        ])
        .output()
        .await
        .expect("Unable to start blobby");
    assert!(
        output.status.success(),
        "Error trying to start blobby {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Get a URL to listen on
    let port = get_random_tcp_port().await;
    let server_url = format!("127.0.0.1:{}", port);

    // Link the things
    let output = Command::new("wash")
        .args([
            "link",
            "put",
            "-p",
            &connection_port,
            id,
            HTTP_SERVER_PROVIDER_ID,
            "wasmcloud:httpserver",
            &format!("ADDRESS={}", server_url),
        ])
        .output()
        .await
        .expect("Unable to link httpserver");
    assert!(
        output.status.success(),
        "Error trying to link http server {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output = Command::new("wash")
        .args([
            "link",
            "put",
            "-p",
            &connection_port,
            id,
            BLOBSTORE_PROVIDER_ID,
            "wasmcloud:blobstore",
            "ROOT=/tmp",
        ])
        .output()
        .await
        .expect("Unable to link blobstore");
    assert!(
        output.status.success(),
        "Error trying to link blobstore {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Make sure the httpserver is ready to test
    wait_for_server(&server_url, &guard).await;

    (server_url, guard)
}
