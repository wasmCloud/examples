use crate::methods::data::{
    get_auth_config, get_auth_url, retrieve_data, store_data, Oauth2Request, Oauth2Response,
};
use serde_json::json;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{debug, error, info, log, warn};

pub async fn login(
    ctx: &Context,
    req: &Oauth2Request,
    path: &str,
) -> Result<Oauth2Response, anyhow::Error> {
    let auth_config = get_auth_config(ctx, req).await?;

    // must be between 32 and 96 bytes
    let login_data = get_auth_url(&auth_config).await?;

    let json = json!(login_data).to_string();

    store_data(ctx, &json, &"unique identifier".to_string()).await?;

    // TODO
    // store_data(ctx, &url_res.csrf_state).await?;
    // To avoid giving away helpful information to an attacker, the API Gateway should return errors of not_found (404) on failure.

    Ok(Oauth2Response {
        auth_url: Some(login_data.url),
        id: Some("unique identifier".to_string()),
        session: None,
        expiry: None,
    })
}
