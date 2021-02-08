#[macro_use]
extern crate wascc_codec as codec;
#[macro_use]
extern crate log;

use payments::{
    AuthorizePaymentRequest, AuthorizePaymentResponse, CompletePaymentRequest,
    CompletePaymentResponse, PaymentMethodList,
};
use payments_interface as payments;

use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, RwLock},
};

use actor_core::{deserialize, serialize, CapabilityConfiguration, HealthCheckResponse};
use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::{OP_BIND_ACTOR, OP_HEALTH_REQUEST, OP_REMOVE_ACTOR};

/// The well-known contract ID for this provider
pub const CAPABILITY_ID: &str = "examples:payments";

#[cfg(not(feature = "static_plugin"))]
capability_provider!(FakePaymentsProvider, FakePaymentsProvider::new);

const OP_AUTHORIZE_PAYMENT: &str = "AuthorizePayment";
const OP_COMPLETE_PAYMENT: &str = "CompletePayment";
const OP_GET_PAYMENT_METHODS: &str = "GetPaymentMethods";

/// An example implementation of the `examples:payments` contract.
#[derive(Clone)]
pub struct FakePaymentsProvider {
    dispatcher: Arc<RwLock<Box<dyn Dispatcher>>>,
}

impl FakePaymentsProvider {
    pub fn new() -> Self {
        let _ = env_logger::try_init();
        FakePaymentsProvider {
            dispatcher: Arc::new(RwLock::new(Box::new(NullDispatcher::new()))),
        }
    }

    fn authorize_payment(
        &self,
        _req: AuthorizePaymentRequest,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        Ok(serialize(&AuthorizePaymentResponse {
            success: true,
            auth_code: Some(uuid::Uuid::new_v4().to_string()),
            fail_reason: None,
        })?)
    }

    fn complete_payment(
        &self,
        _req: CompletePaymentRequest,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        Ok(serialize(&CompletePaymentResponse {
            success: true,
            txid: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        })?)
    }

    fn get_payment_methods(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        Ok(serialize(&PaymentMethodList {
            methods: fake_methods(),
        })?)
    }
}

impl CapabilityProvider for FakePaymentsProvider {
    fn configure_dispatch(
        &self,
        dispatcher: Box<dyn Dispatcher>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Dispatcher configured.");

        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    fn handle_call(
        &self,
        actor: &str,
        op: &str,
        msg: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        trace!("Handling operation `{}` from `{}`", op, actor);

        match op {
            OP_BIND_ACTOR if actor == "system" => {
                // Provision per-actor resources here
                Ok(vec![])
            }
            OP_REMOVE_ACTOR if actor == "system" => {
                let cfgvals = deserialize::<CapabilityConfiguration>(msg)?;
                info!("Removing actor configuration for {}", cfgvals.module);
                // Clean up per-actor resources here
                Ok(vec![])
            }
            OP_HEALTH_REQUEST if actor == "system" => Ok(serialize(HealthCheckResponse {
                healthy: true,
                message: "".to_string(),
            })?),

            // contract-specific handlers
            OP_AUTHORIZE_PAYMENT => self.authorize_payment(deserialize(&msg)?),
            OP_COMPLETE_PAYMENT => self.complete_payment(deserialize(&msg)?),
            OP_GET_PAYMENT_METHODS => self.get_payment_methods(),
            _ => Err(format!("Unsupported operation `{}`", op).into()),
        }
    }

    fn stop(&self) {
        // Do nothing
    }
}

fn fake_methods() -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("token1".to_string(), "PayBuddy".to_string());
    hm.insert("token2".to_string(), "Bank of Exemplar".to_string());

    hm
}
