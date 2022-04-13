use wasmbus_rpc::{common::AnySender, provider::prelude::*};
use wasmcloud_test_util::{
    cli::print_test_results,
    provider_test::test_provider,
    run_selected_spawn,
    testing::{TestOptions, TestResult},
};

#[tokio::test]
async fn run_all() {
    // load the first time
    let prov = test_provider().await;
    let opts = TestOptions::default();

    let res = run_selected_spawn!(&opts, test_basic);
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.passed).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    // try to let the provider shut down gracefully
    if let Err(e) = prov.shutdown().await {
        eprintln!("ERROR provider shutdown returned: {}", e.to_string());
    }
}

/// invoke through wasmbus rpc
async fn test_basic(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    let py = AnySender::new(prov);
    let ctx = Context::default();

    let n: i32 = 10;
    let res: i32 = py.send(&ctx, "f.factorial", &n).await?;
    assert_eq!(res, 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2);

    let name = "Alice".to_string();
    let res: String = py.send(&ctx, "hello", &name).await?;
    assert_eq!(res.as_str(), "Hello Alice!");

    Ok(())
}
