use petclinic_interface::{GetAssetResponse, Ui, UiReceiver};
use rust_embed::RustEmbed;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::debug;

#[derive(RustEmbed)]
#[folder = "./dist"]
struct Asset;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Ui)]
struct UiActor {}

#[async_trait]
impl Ui for UiActor {
    async fn get_asset<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        _ctx: &Context,
        path: &TS,
    ) -> RpcResult<GetAssetResponse> {
        let path = path.to_string();
        let trimmed = if path.trim() == "/" {
            // Default to index.html if the root path is given alone
            debug!("Found root path, assuming index.html");
            "index.html"
        } else {
            path.trim().trim_start_matches('/')
        };

        debug!("Got path {}, attempting to fetch", trimmed);
        if let Some(file) = Asset::get(trimmed) {
            debug!(
                "Found file {}, returning {} bytes",
                trimmed,
                file.data.len()
            );
            return Ok(GetAssetResponse {
                found: true,
                asset: Vec::from(file.data),
                content_type: mime_guess::from_path(trimmed)
                    .first()
                    .map(|m| m.to_string()),
            });
        }

        debug!("Did not find file {}, returning", trimmed);
        return Ok(GetAssetResponse {
            found: false,
            ..Default::default()
        });
    }
}
