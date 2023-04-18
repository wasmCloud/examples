use crate::anyhow;
use crate::methods::data::{
    compare_state, generate_auth_string, get_auth_config, retrieve_data, store_data, AuthConfig,
    LoginData, Oauth2Request, Oauth2Response,
};
use serde_json::json;
use serde_urlencoded;
use wasmbus_rpc::actor::prelude::Context;
use wasmcloud_interface_httpclient::*;
use wasmcloud_interface_logging::{debug, error, info, log, warn};

pub async fn login_callback(
    ctx: &Context,
    req: &Oauth2Request,
) -> Result<Oauth2Response, anyhow::Error> {
    let auth_config = get_auth_config(ctx, req).await?;

    let login_data = retrieve_data(ctx, "unique identifier").await?;

    let state_result = compare_state(&login_data.csrf_state, &req.state.clone().unwrap()).await?;

    match state_result {
        true => {
            let res = token_exchange(ctx, req, &auth_config, &login_data).await?;
            info!("{:?}", res);
            let json = json!(res).to_string();
            store_data(ctx, &json, &"unique identifier".to_string()).await?;
            Ok(Oauth2Response {
                auth_url: None,
                id: Some("unique identifier".to_string()),
                session: Some("Session Number".to_string()),
                expiry: None,
            })
        }
        false => Err(anyhow!("Token Exchange Failed")),
    }

    // authorize_user().await?;
    // store_data().await?;
}

// exchange the authorization code for an access token
// convert this to httpclient request
// may need to update to handle more oauth providers
async fn token_exchange(
    ctx: &Context,
    req: &Oauth2Request,
    conf: &AuthConfig,
    data: &LoginData,
) -> Result<HttpResponse, anyhow::Error> {
    // application/x-www-form-urlencoded Mime type body
    let body = serde_urlencoded::to_string([
        ("grant_type", "authorization_code"),
        ("code", req.code.clone().unwrap().as_str()),
        ("redirect_uri", conf.redirect_url.as_str()),
        ("code_verifier", data.code_verifier.as_str()),
        ("client_id", conf.client_id.as_str()),
        // unnecessary for Spotify, may be necessary for other Oauth providers.
        // ("client_secret", conf.client_secret.as_str()),
    ])?;

    let body = body.as_bytes().to_vec();

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "Authorization".to_string(),
        // Base 64 encoded string that contains the client ID and client secret key. The field must have the format: Authorization: Basic <base64 encoded client_id:client_secret>
        vec![format!("Basic {}", generate_auth_string(conf).await?)],
    );
    headers.insert(
        "Content-Type".to_string(),
        vec!["application/x-www-form-urlencoded".to_string()],
    );

    let client = HttpClientSender::new();
    let res = client
        .request(
            ctx,
            &HttpRequest {
                method: "POST".to_string(),
                headers: headers,
                url: conf.token_url.clone(),
                body: body,
            },
        )
        .await?;

    Ok(res)
}

async fn authorize_user() -> Result<(), anyhow::Error> {
    todo!();
    // generate_client_state();
    // store_data();
}
