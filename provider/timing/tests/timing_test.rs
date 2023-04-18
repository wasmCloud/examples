use std::ops::Add;
use std::time::{Duration, SystemTime};
use wasmbus_rpc::error::RpcResult;
use wasmbus_rpc::provider::prelude::*;
use wasmbus_rpc::Timestamp;
use wasmcloud_test_util::{check, cli::print_test_results, provider_test::test_provider, testing::TestOptions};
#[allow(unused_imports)]
use wasmcloud_test_util::{run_selected, run_selected_spawn};
use wasmcloud_interface_timing::{Timing, TimingSender};

#[tokio::test]
async fn run_all() {
    let opts = TestOptions::default();
    let res = run_selected_spawn!(&opts, test_health_check, test_sleep, test_sleep_until, test_now);
    print_test_results(&res);

    let passed = res.iter().filter(|tr| tr.passed).count();
    let total = res.len();
    assert_eq!(passed, total, "{} passed out of {}", passed, total);

    // try to let the provider shut down gracefully
    let provider = test_provider().await;
    let _ = provider.shutdown().await;
}

/// test that health check returns healthy
async fn test_health_check(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    // health check
    let hc = prov.health_check().await;
    check!(hc.is_ok())?;
    Ok(())
}

/// test that `TimingSender::sleep()` works correctly
async fn test_sleep(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    let client = TimingSender::via(prov);
    let ctx = Context::default();

    let start = tokio::time::Instant::now();
    let sleep_time_ms = 100;
    client.sleep(&ctx, &sleep_time_ms).await?;
    let actual_time_slept = start.elapsed();

    check!(
        actual_time_slept >= Duration::from_millis(sleep_time_ms as u64)
    )?;

    Ok(())
}

/// test that `TimingSender::sleep_until()` works correctly
async fn test_sleep_until(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    let client = TimingSender::via(prov);
    let ctx = Context::default();

    let start = tokio::time::Instant::now();
    let sleep_duration = Duration::from_millis(100);
    let sys_timestamp = SystemTime::now();
    let sleep_until = Timestamp::from(sys_timestamp.add(sleep_duration));
    client.sleep_until(&ctx, &sleep_until).await?;
    let actual_time_slept = start.elapsed();

    check!(
        actual_time_slept >= sleep_duration
    )?;

    Ok(())
}

/// test that `TimingSender::now()` works correctly
async fn test_now(_opt: &TestOptions) -> RpcResult<()> {
    let prov = test_provider().await;

    let client = TimingSender::via(prov);
    let ctx = Context::default();

    let start = client.now(&ctx).await?;
    let sleep_duration = Duration::from_millis(100);
    tokio::time::sleep(sleep_duration).await;
    let end = client.now(&ctx).await?;

    // check that the difference between the start and end times is within 10ms of the sleep duration
    check!((end.as_nanos() - start.as_nanos()).abs_diff(sleep_duration.as_nanos()) < 10_000_000)?;

    Ok(())
}
