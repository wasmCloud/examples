use wasmbus_rpc::provider::prelude::*;
use wasmcloud_interface_factorial::*;
use wasmcloud_test_util::{
    check,
    cli::print_test_results,
    provider_test::test_provider,
    testing::{TestOptions, TestResult},
};
#[allow(unused_imports)]
use wasmcloud_test_util::{run_selected, run_selected_spawn};

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(&opts, health_check, factorial_0_1, factorial_more);
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.passed).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    // try to let the provider shut dowwn gracefully
    let provider = test_provider().await;
    let _ = provider.shutdown().await;
}

/// test that health check returns healthy
async fn health_check(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // health check
    let hc = prov.health_check().await;
    check!(hc.is_ok())?;
    Ok(())
}

/// Factorial tests
async fn factorial_0_1(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // create client and ctx
    let client = FactorialSender::via(prov);
    let ctx = Context::default();

    let resp = client.calculate(&ctx, &0).await?;
    assert_eq!(resp, 1, "0!");

    let resp = client.calculate(&ctx, &1).await?;
    assert_eq!(resp, 1, "1!");

    Ok(())
}

/// Factorial tests
async fn factorial_more(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // create client and ctx
    let client = FactorialSender::via(prov);
    let ctx = Context::default();

    let resp = client.calculate(&ctx, &2).await?;
    assert_eq!(resp, 2, "2!");

    let resp = client.calculate(&ctx, &3).await?;
    assert_eq!(resp, 6, "3!");

    let resp = client.calculate(&ctx, &4).await?;
    assert_eq!(resp, 24, "4!");

    Ok(())
}
