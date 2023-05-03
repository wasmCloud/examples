/// Configuration Service Example
///
/// This is an EXAMPLE only. This is not code you should be running in production
/// because it lacks basic security properties, is not optimized for highly
/// concurrent environments, etc.
///
/// This _example_ illustrates how to make a service that responds to
/// wasmbus.cfg.{lattice}.req, wasmbus.cfg.{lattice}.put, and publish
/// credential map changes to wasmbus.ctl.{lattice}.registries.put.
///
/// This sample is also chock full of unwraps where there shouldn't be any, so
/// please do not follow that pattern.
use lazy_static::lazy_static;
use std::{collections::HashMap, env::var, error::Error, sync::RwLock, thread};

use serde::{Deserialize, Serialize};
use tracing::info;

lazy_static! {
    static ref CREDENTIAL_MAP: RwLock<HashMap<String, Credential>> = RwLock::new(HashMap::new());
    static ref AUTOSTART_ACTORS: RwLock<Vec<AutoStartActorsProfile>> = RwLock::new(Vec::new());
    static ref AUTOSTART_PROVIDERS: RwLock<Vec<AutoStartProvidersProfile>> =
        RwLock::new(Vec::new());
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_ansi(atty::is(atty::Stream::Stderr))
        .init();

    fill_autostarts();
    info!("Started SAMPLE config server with {} autostart actor profiles, {} autostart provider profiles, and {} artifact registries",
        AUTOSTART_ACTORS.read().unwrap().len(),
        AUTOSTART_PROVIDERS.read().unwrap().len(),
        CREDENTIAL_MAP.read().unwrap().len());

    let lattice_id = var("LATTICE_ID").unwrap_or("default".to_string());

    // Note: this connection will need to be able to access the control interface
    // topics. Also, you don't want to use an unsecure connection like this in
    // production
    let conn = nats::connect("127.0.0.1")?;
    let c = conn.clone();
    let topic = format!("wasmbus.cfg.{}.req", lattice_id);
    conn.subscribe(&topic)?.with_handler(move |msg| {
        let req: ConfigurationRequest = serde_json::from_slice(&msg.data).unwrap();
        info!(
            "Received request on '{}' for labels: {:?}",
            topic, req.labels
        );
        let response = generate_response(&req);
        let _ = msg
            .respond(&serde_json::to_vec(&response).unwrap())
            .unwrap();
        Ok(())
    });

    let put_topic = format!("wasmbus.cfg.{}.put", lattice_id);
    let ctl_topic = format!("wasmbus.ctl.{}.registries.put", lattice_id);
    conn.subscribe(&put_topic)?.with_handler(move |msg| {
        let m: HashMap<String, Credential> = serde_json::from_slice(&msg.data).unwrap();
        info!("Receiving replacement registry map: {} registries", m.len());
        *CREDENTIAL_MAP.write().unwrap() = m;

        c.publish(&ctl_topic, &msg.data).unwrap();
        Ok(())
    });

    thread::park();
    Ok(())
}

fn fill_autostarts() {
    let mut petclinic_labels = HashMap::new();
    petclinic_labels.insert("app".to_string(), "petclinic".to_string());
    let petclinic_actors = AutoStartActorsProfile {
        requirements: petclinic_labels.clone(),
        actors: vec![
            "wasmcloud.azurecr.io/clinicapi:0.3.4".to_string(),
            "wasmcloud.azurecr.io/vets:0.3.4".to_string(),
            "wasmcloud.azurecr.io/visits:0.3.4".to_string(),
            "wasmcloud.azurecr.io/customers:0.3.4".to_string(),
        ],
    };

    AUTOSTART_ACTORS.write().unwrap().push(petclinic_actors);

    let petclinic_providers = AutoStartProvidersProfile {
        requirements: petclinic_labels,
        providers: vec![
            ProviderReference {
                image_reference: "wasmcloud.azurecr.io/httpserver:0.17.0".to_string(),
                link_name: "default".to_string(),
            },
            ProviderReference {
                image_reference: "wasmcloud.azurecr.io/sqldb-postgres:0.5.0".to_string(),
                link_name: "default".to_string(),
            },
        ],
    };

    AUTOSTART_PROVIDERS
        .write()
        .unwrap()
        .push(petclinic_providers);
}

fn generate_response(req: &ConfigurationRequest) -> HostConfigurationProfile {
    let actors: Vec<_> = {
        let lock = AUTOSTART_ACTORS.read().unwrap();
        lock.iter()
            .filter(|&aap| satisfies_requirements(&req.labels, &aap.requirements))
            .flat_map(|profile| profile.actors.clone())
            .collect()
    };
    let providers: Vec<_> = {
        let lock = AUTOSTART_PROVIDERS.read().unwrap();
        lock.iter()
            .filter(|&app| satisfies_requirements(&req.labels, &app.requirements))
            .flat_map(|profile| profile.providers.clone())
            .collect()
    };

    HostConfigurationProfile {
        auto_start_actors: actors,
        auto_start_providers: providers,
        registry_credentials: CREDENTIAL_MAP.read().unwrap().clone(),
    }
}

// If all of the actual values from the profile exist inside the label dump
// from the request, then the profile is suitable for this target.
// So if a request comes in with the `hostcore.*` labels, a `foo=bar` label,
// and the `app=petclinic` label, and the actual/profile contains 'app=petclinic',
// then this function will return true.
// You may want to apply a different policy to how you evalute the incoming
// requirements (which is a full dump of all labels on the host) against the
// profile.
//
// note: Map 'actual' is a subset of map 'req' if req.merge(actual) == req
fn satisfies_requirements(req: &HashMap<String, String>, actual: &HashMap<String, String>) -> bool {
    let mut req2 = req.clone();
    req2.extend(actual.clone());
    req2 == *req
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Credential {
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "registryType")]
    pub registry_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HostConfigurationProfile {
    #[serde(rename = "autoStartProviders")]
    pub auto_start_providers: Vec<ProviderReference>,

    #[serde(rename = "autoStartActors")]
    pub auto_start_actors: Vec<String>,

    #[serde(rename = "registryCredentials")]
    pub registry_credentials: HashMap<String, Credential>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderReference {
    #[serde(rename = "linkName")]
    pub link_name: String,

    #[serde(rename = "imageReference")]
    pub image_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AutoStartActorsProfile {
    pub requirements: HashMap<String, String>,
    pub actors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AutoStartProvidersProfile {
    pub requirements: HashMap<String, String>,
    pub providers: Vec<ProviderReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConfigurationRequest {
    labels: HashMap<String, String>,
}
