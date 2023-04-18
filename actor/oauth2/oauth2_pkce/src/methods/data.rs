#![allow(unused_imports)]
use anyhow::Error;
use base64::{
    engine::{
        self,
        general_purpose::{self, URL_SAFE_NO_PAD},
    },
    Engine as _,
};
use jammin_interfaces_apigw::RoutedRequest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::clone::Clone;
use std::fmt::Debug;
use url::Url;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::HttpRequest;
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender, SetRequest};
use wasmcloud_interface_logging::{debug, error, info, log, warn};
use wasmcloud_interface_numbergen::{generate_guid, random_32, random_in_range};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Oauth2Request {
    pub provider: String,
    pub grant: String,
    pub code: Option<String>,
    pub state: Option<String>,
    pub id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Oauth2Response {
    pub auth_url: Option<String>,
    pub id: Option<String>,
    pub session: Option<String>,
    pub expiry: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    // Option so that if None can return error
    grant_type: Option<String>,
    pub client_id: String,
    pub client_secret: String,
    auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
    scope: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LoginData {
    pub url: String,
    pub csrf_state: String,
    code_challenge: String,
    pub code_verifier: String,
}

// store  making sure email, and their various provider auth settings can be saved
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LoginCallbackData {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: String,
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Data {
    Login,
    LoginCallback,
}

// function to get specific auth provider configuration from KV-Vault
pub async fn get_auth_config(
    ctx: &Context,
    req: &Oauth2Request,
) -> Result<AuthConfig, anyhow::Error> {
    // get key based on req parameter field for social provider defined above
    let res = KeyValueSender::new_with_link("vault")?
        .get(ctx, &req.provider)
        .await?;

    // res contains JSON structured as follows: {"auth_url":"some_url","client_id":"some_id","client_secret":"some_secret","redirect_url":"some_callback","scope":"some_scope","token_url":"some_token_url"}
    let mut config: AuthConfig = serde_json::from_str(res.value.as_str())?;
    config.grant_type = Some(req.grant.clone());

    Ok(config)
}

pub async fn get_auth_url(config: &AuthConfig) -> Result<LoginData, anyhow::Error> {
    // may need to generate larger state number
    let state: String = general_purpose::URL_SAFE_NO_PAD.encode(generate_long_random(80).await?);

    // must be between 32 and 96 bytes per [RFC 7636](https://tools.ietf.org/html/rfc7636)
    // dividing by for because 4 octets in 32 bit number
    let code_verifier: String =
        general_purpose::URL_SAFE_NO_PAD.encode(generate_long_random(80).await?);
    let code_challenge: String =
        general_purpose::URL_SAFE_NO_PAD.encode(Sha256::digest(&code_verifier));

    let mut auth_url = Url::parse(config.auth_url.as_str()).unwrap();
    auth_url
        .query_pairs_mut()
        .append_pair("client_id", config.client_id.as_str())
        .append_pair("redirect_uri", config.redirect_url.as_str())
        // response type must be token or code
        .append_pair("response_type", "code")
        .append_pair("scope", config.scope.as_str())
        .append_pair(
            "state",
            general_purpose::URL_SAFE_NO_PAD.encode(&state).as_str(),
        )
        .append_pair("code_challenge", code_challenge.as_str())
        .append_pair("code_challenge_method", "S256");

    let res = LoginData {
        url: auth_url.to_string(),
        csrf_state: state,
        code_challenge: code_challenge,
        code_verifier: code_verifier,
    };
    Ok(res)
}

pub async fn compare_state(csrf_state: &String, req_state: &String) -> Result<bool, anyhow::Error> {
    let decoded_state = general_purpose::URL_SAFE_NO_PAD.decode(req_state)?;
    let received_state = std::str::from_utf8(&decoded_state)?;

    // compare state from redirect response with state created to make initial auth req
    if csrf_state.as_str() != received_state {
        info!("error");
        Ok(false)
    } else {
        Ok(true)
    }
}

async fn generate_long_random(bytes: u32) -> Result<Vec<u8>, anyhow::Error> {
    assert!(bytes / 4 >= 32 / 4 && bytes / 4 <= 96 / 4);
    let mut random_bytes: Vec<u8> = Vec::new();
    // Add several u32 numbers into the vector using a for loop
    for _i in 0..bytes / 4 {
        let num: u32 = random_32().await?; // generate a number
        let bytes = num.to_le_bytes(); // convert number to little-endian bytes
        random_bytes.extend_from_slice(&bytes); // add bytes into the vector
    }
    Ok(random_bytes)
}

// Generates a unique authentication state value to return to the client, has expiry
pub async fn generate_auth_string(req: &AuthConfig) -> Result<String, anyhow::Error> {
    let string = format!("{}:{}", req.client_id, req.client_secret);
    let mut encoded_string = String::new();
    general_purpose::URL_SAFE_NO_PAD.encode_string(string, &mut encoded_string);

    Ok(encoded_string)
}

// Check if a GUID already exists for a user, if not create one
pub async fn check_user_id() -> Result<String, anyhow::Error> {
    // NEED TO MAKE SURE IT IS GLOBALLY UNIQUE FOR ALL USERS
    todo!();

    // check if user exists -> if exists get user id -> if not exists generate id
}

// First time user performs token exhange, a GUID is generated
pub async fn generate_user_id() -> Result<String, anyhow::Error> {
    // NEED TO MAKE SURE IT IS GLOBALLY UNIQUE FOR ALL USERS
    let user_id: String = generate_guid().await?; // Should this be random in range

    // check if user_state_id is unique -> if not recreate -> if unique store and return
    Ok(user_id)
}

// Generates a unique authentication state value to return to the client, has expiry
pub async fn generate_user_state() -> Result<String, anyhow::Error> {
    // NEED TO MAKE SURE IT IS GLOBALLY UNIQUE FOR ALL USERS
    todo!();
    // let user_state_id: String = generate_guid().await?; // Should this be random in range

    // check if user_state_id exists -> if yes, recreate -> if unique store and return
}

// function to store relevant data from processes - keep generic and pass in data to be stored as parameters
// return true if success and false if fail
pub async fn store_data(ctx: &Context, json: &String, user: &String) -> Result<(), RpcError> {
    let arg = SetRequest {
        key: user.clone(),
        value: json.clone(),
        expires: 0,
    };

    let res = KeyValueSender::new_with_link("redis")?
        .set(ctx, &arg)
        .await?;

    Ok(res)
}

pub async fn retrieve_data(ctx: &Context, data: &str) -> Result<LoginData, anyhow::Error> {
    let val = KeyValueSender::new_with_link("redis")?
        .get(ctx, data)
        .await?
        .value;

    let res: LoginData = serde_json::from_str(&val)?;

    Ok(res)
}

pub async fn salt_data(ctx: &Context) -> Result<(), anyhow::Error> {
    todo!();
    // use numbergen
    // shouldn't need to sign csrf_state, sign tokens and store salt beside hashed value
}
