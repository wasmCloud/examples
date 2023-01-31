use tokio::process::Command;

const HTTP_SERVER_PROVIDER_ID: &str = "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M";
const BLOBSTORE_PROVIDER_ID: &str = "VBBQNNCGUKIXEWLL5HL5XJE57BS3GU5DMDOKZS6ROEWPQFHEDP6NGVZM";
const DOCKER_REGISTRY_NAME: &str = "blobby-test";
const REGISTRY_REF: &str = "localhost:9999/blobby:dev";
const BLOBBY_PATH: &str = "../actor/build/blobby_s.wasm";
const WASHBOARD_URL: &str = "localhost:4000";
const SERVER_URL: &str = "127.0.0.1:8080";

pub struct CleanupGuard {
    already_running: bool,
}

impl Drop for CleanupGuard {
    fn drop(&mut self) {
        match std::process::Command::new("docker")
            .args(["rm", "-f", DOCKER_REGISTRY_NAME])
            .output()
        {
            Ok(o) if !o.status.success() => {
                eprintln!(
                    "Error stopping docker container: {}",
                    String::from_utf8_lossy(&o.stderr)
                )
            }
            Err(e) => eprintln!("Error stopping docker container: {}", e),
            _ => (),
        }

        if !self.already_running {
            match std::process::Command::new("wash").args(["down"]).output() {
                Ok(o) if !o.status.success() => {
                    eprintln!(
                        "Error stopping wasmcloud host: {}",
                        String::from_utf8_lossy(&o.stderr)
                    )
                }
                Err(e) => eprintln!("Error stopping wasmcloud host: {}", e),
                _ => (),
            }
        }
    }
}

// TODO: Make this actually be unique for each test so we can run in parallel
pub async fn setup_test() -> (String, CleanupGuard) {
    // Build module
    let output = Command::new("wash")
        .arg("build")
        .current_dir("../actor")
        .status()
        .await
        .expect("Unable to run wash build");
    assert!(output.success(), "Error trying to build module",);

    // Get the actor ID
    let output = Command::new("wash")
        .args(["claims", "inspect", "-o", "json", BLOBBY_PATH])
        .output()
        .await
        .expect("Unable to start docker registry");
    assert!(
        output.status.success(),
        "Unable to start docker registry {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let claims = serde_json::from_slice::<serde_json::Value>(&output.stdout)
        .expect("Unable to decode claims");
    let id = claims
        .get("module")
        .and_then(|v| v.as_str())
        .expect("Couldn't find module ID");

    // Temporary step: start docker registry until starting from a file is supported and push to
    // registry: https://github.com/wasmCloud/wasmcloud-otp/pull/529
    let _ = Command::new("docker")
        .args(["rm", "-f", DOCKER_REGISTRY_NAME])
        .output()
        .await
        .expect("Unable to start docker registry");
    let output = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-d",
            "--name",
            DOCKER_REGISTRY_NAME,
            "-p",
            "9999:5000",
            "registry:2",
        ])
        .output()
        .await
        .expect("Unable to start docker registry");
    assert!(
        output.status.success(),
        "Unable to start docker registry {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // If we push to fast, we catch the server with its pants down. Just trying to connect to the
    // registry in a loop didn't work
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let output = Command::new("wash")
        .args(["reg", "push", "--insecure", REGISTRY_REF, BLOBBY_PATH])
        .output()
        .await
        .expect("Unable to push module");
    assert!(
        output.status.success(),
        "Unable to push module {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // NOTE (thomastaylor312): I decided just to use wash to start everything since it does the
    // waiting for me and it mimics what a user would do IRL

    // Start wasmcloud host if we don't find one running
    let already_running = if tokio::net::TcpStream::connect(WASHBOARD_URL).await.is_err() {
        let output = Command::new("wash")
            .args(["up", "-d"])
            .env("WASMCLOUD_OCI_ALLOWED_INSECURE", "localhost:9999")
            .status()
            .await
            .expect("Unable to run wash up");
        assert!(output.success(), "Error trying to start host",);
        // Make sure we can connect
        wait_for_server(WASHBOARD_URL).await;
        false
    } else {
        true
    };

    // TODO: Be idempotent starting providers

    // Start http server
    let output = Command::new("wash")
        .args([
            "ctl",
            "start",
            "provider",
            "wasmcloud.azurecr.io/httpserver:0.16.0",
        ])
        .status()
        .await
        .expect("Unable to start http provider");
    assert!(output.success(), "Error trying to start http provider",);

    // Start blobstore_fs
    let output = Command::new("wash")
        .args([
            "ctl",
            "start",
            "provider",
            "wasmcloud.azurecr.io/blobstore_fs:0.2.0",
        ])
        .status()
        .await
        .expect("Unable to start blobstore provider");
    assert!(output.success(), "Error trying to start blobstore provider",);

    // Start module
    let output = Command::new("wash")
        .args(["ctl", "start", "actor", REGISTRY_REF])
        .output()
        .await
        .expect("Unable to start blobby");
    assert!(
        output.status.success(),
        "Error trying to start blobby {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Link the things
    let output = Command::new("wash")
        .args([
            "ctl",
            "link",
            "put",
            id,
            HTTP_SERVER_PROVIDER_ID,
            "wasmcloud:httpserver",
            &format!("ADDRESS={}", SERVER_URL),
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
            "ctl",
            "link",
            "put",
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
    wait_for_server(SERVER_URL).await;

    (SERVER_URL.to_owned(), CleanupGuard { already_running })
}

async fn wait_for_server(url: &str) {
    let mut wait_count = 1;
    loop {
        // Magic number: 10 + 1, since we are starting at 1 for humans
        if wait_count >= 11 {
            panic!("Ran out of retries waiting for host to start");
        }
        match tokio::net::TcpStream::connect(url).await {
            Ok(_) => break,
            Err(e) => {
                eprintln!("Waiting for server {} to come up, attempt {}. Will retry in 1 second. Got error {:?}", url, wait_count, e);
                wait_count += 1;
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
}
