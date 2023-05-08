//! cron capability provider
//!
use std::{collections::HashMap, str::FromStr, time::Duration};

use cron::Schedule;
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_interface_cron::{Cron, CronSender};

const EXPRESSION: &str = "expression";
const SECOND: &str = "second";
const MINUTE: &str = "minute";
const HOUR: &str = "hour";
const DAY: &str = "day";
const MONTH: &str = "month";
const DAY_OF_WEEK: &str = "day_of_week";
const YEAR: &str = "year";
const DEFAULT_CRON_VALUE: &str = "*";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(CronProvider::default(), Some("Cron".to_string()))?;

    eprintln!("cron provider exiting");
    Ok(())
}

/// cron capability provider implementation
#[derive(Default, Clone, Provider)]
// TODO: Map from actor ID to join handle
struct CronProvider {
    // handles: HashMap<String, JoinHandle<()>>,
}

/// use default implementations of provider message handlers
impl ProviderDispatch for CronProvider {}
#[async_trait]
impl ProviderHandler for CronProvider {
    async fn put_link(&self, ld: &LinkDefinition) -> RpcResult<bool> {
        let ld = ld.clone();
        let schedule = if let Ok(schedule) = cron_schedule(&ld.values) {
            schedule
        } else {
            // If we fail to parse the schedule, deny link definition
            return Ok(false);
        };
        tokio::spawn(async move {
            let mut prev = chrono::Utc::now();
            loop {
                // Get the next cron occurrence
                // SAFETY: If we fail to get the next interval, we can't continue to tick, so this task can panic
                let next = schedule.upcoming(chrono::Utc).next().unwrap();
                tracing::debug!("Next invocation for actor {} at {next}", ld.actor_id);

                // Avoid triggering multiple invocations on the same occurrence
                if next == prev {
                    continue;
                }

                // Calculate the duration until the next occurrence
                let duration = next - chrono::Utc::now();

                // Sleep for the duration
                tokio::time::sleep(Duration::from_secs(duration.num_seconds() as u64)).await;

                // let coarse_time = (chrono::Utc::now().timestamp() / 10) * 10;
                let coarse_time = chrono::Utc::now().timestamp();

                // Call your function here
                if let Err(e) = CronSender::for_actor(&ld)
                    .timed_invoke(&Context::default(), &(coarse_time as u64))
                    .await
                {
                    tracing::error!("Error time invoking actor {} : {e}", ld.actor_id)
                };

                prev = next;
            }
        });

        Ok(true)
    }
}

/// Parses a cron expression from a HashMap of key-value pairs.
///
/// The function expects a HashMap of String keys and values, where the keys correspond to the
/// names of the cron fields ("second", "minute", "hour", etc.) and the values are strings that
/// represent the values of those fields. If the HashMap contains an "expression" key, its value
/// is used as the cron expression; otherwise, the function constructs a cron expression from
/// the individual fields.
///
/// # Arguments
///
/// * `ld_values` - A reference to a HashMap of String keys and values that represent a cron
/// expression or its individual fields, usually sourced from the [LinkDefinition](LinkDefinition).
///
/// # Returns
///
/// * If the function successfully parses a cron expression from the input HashMap, it returns
/// a `Schedule` object that represents the cron schedule.
///
/// * If the cron expression is invalid, the function returns an `Error` object.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use cron::Schedule;
///
/// let mut ld_values = HashMap::new();
/// ld_values.insert("minute".to_string(), "*/5".to_string());
/// ld_values.insert("hour".to_string(), "*".to_string());
///
/// let schedule = cron_schedule(&ld_values).unwrap();
/// assert_eq!(schedule.upcoming_utc(chrono::Utc).take(5).count(), 5);
/// ```
fn cron_schedule(ld_values: &HashMap<String, String>) -> Result<Schedule, cron::error::Error> {
    // Custom expressions take precedent over individual values
    if let Some(expression) = ld_values.get(EXPRESSION) {
        Schedule::from_str(expression)
    } else {
        let cron_star = DEFAULT_CRON_VALUE.to_string();
        Schedule::from_str(&format!(
            "{} {} {} {} {} {} {}",
            ld_values.get(SECOND).unwrap_or(&cron_star),
            ld_values.get(MINUTE).unwrap_or(&cron_star),
            ld_values.get(HOUR).unwrap_or(&cron_star),
            ld_values.get(DAY).unwrap_or(&cron_star),
            ld_values.get(MONTH).unwrap_or(&cron_star),
            ld_values.get(DAY_OF_WEEK).unwrap_or(&cron_star),
            ld_values.get(YEAR).unwrap_or(&cron_star)
        ))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::cron_schedule;

    #[test]
    fn can_make_scheduler() {
        let values = HashMap::from_iter([("second".to_string(), "".to_string())]);
        let scheduler = cron_schedule(&values);
        assert!(scheduler.is_ok());

        let expression =
            HashMap::from_iter([("expression".to_string(), "1 * * * * * *".to_string())]);

        let scheduler = cron_schedule(&expression);
        assert!(scheduler.is_ok())
    }
}
