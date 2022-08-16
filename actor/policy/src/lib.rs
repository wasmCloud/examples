use serde_json::Value;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_messaging::{
    MessageSubscriber, MessageSubscriberReceiver, Messaging, MessagingSender, PubMessage,
    SubMessage,
};

// The official wasmCloud issuer (this is a public key)
const WASMCLOUD_ISSUER: &str = "ACOJJN6WUP4ODD75XEBKKTCCUJJCY5ZKQ56XVKYK4BEJWGVAOOQHZMCW";
// These are the actions that this actor actually cares to evaluate, other actions can be allowed by default
const START_ACTIONS: [&str; 2] = ["start_actor", "start_provider"];

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber)]
struct PolicyActor {}

#[async_trait]
impl MessageSubscriber for PolicyActor {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        match (msg.reply_to.as_ref(), serde_json::from_slice(&msg.body)) {
            (Some(subject), Ok(Value::Object(policy_request))) => {
                // Here we're manually retrieving keys from the policy struct to avoid writing a full policy request struct
                let policy_result = match (
                    policy_request["requestId"].clone(),
                    policy_request["action"].clone(),
                ) {
                    (Value::String(request_id), Value::String(action)) => {
                        if START_ACTIONS.contains(&action.as_str()) {
                            if let Value::String(target_issuer) =
                                policy_request["target"]["issuer"].clone()
                            {
                                evaluate_issuer(target_issuer.as_str(), &request_id)
                            } else {
                                PolicyResult::allow(None, &request_id)
                            }
                        } else {
                            PolicyResult::allow(None, &request_id)
                        }
                    }
                    (Value::String(request_id), _) => PolicyResult::allow(None, &request_id),
                    (_, _) => PolicyResult::allow(None, ""),
                };

                let body = serde_json::to_vec(&policy_result)
                    .map_err(|_e| RpcError::Ser("Failed to serialize policy result".into()))?;
                if let Err(e) = MessagingSender::new()
                    .publish(
                        ctx,
                        &PubMessage {
                            body,
                            subject: subject.to_owned(),
                            reply_to: None,
                        },
                    )
                    .await
                {
                    Err(format!("Could not publish message {}", e.to_string()).into())
                } else {
                    Ok(())
                }
            }
            (_, _) => Ok(()),
        }
    }
}

/// Deny the issuer if it's not the official wasmCloud issuer, otherwise allow
fn evaluate_issuer(issuer: &str, request_id: &str) -> PolicyResult {
    if issuer == WASMCLOUD_ISSUER {
        PolicyResult::allow(None, request_id)
    } else {
        PolicyResult::deny(
            Some("Issuer was not the official wasmCloud issuer".to_string()),
            request_id,
        )
    }
}

/// The PolicyResult struct that contains necessary information to give an allow
/// or deny an action. This, if needed in the future, should probably be defined in smithy
/// along with the policy request struct
#[derive(serde::Serialize)]
struct PolicyResult {
    permitted: bool,
    message: Option<String>,
    request_id: String,
}

impl PolicyResult {
    fn allow(message: Option<String>, request_id: &str) -> Self {
        PolicyResult {
            permitted: true,
            message,
            request_id: request_id.to_owned(),
        }
    }
    fn deny(message: Option<String>, request_id: &str) -> Self {
        PolicyResult {
            permitted: false,
            message,
            request_id: request_id.to_owned(),
        }
    }
}
