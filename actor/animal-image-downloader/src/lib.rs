use serde_json::Value;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_blobstore::{Blobstore, BlobstoreSender, Chunk, PutObjectRequest};
use wasmcloud_interface_httpclient::{HttpClient, HttpClientSender, HttpRequest, HttpResponse};
use wasmcloud_interface_messaging::{
    MessageSubscriber, MessageSubscriberReceiver, Messaging, MessagingSender, PubMessage,
    SubMessage,
};

const REQUEST_PREFIX: &str = "wasmcloud.animal.";
const CAT_URL: &str = "https://api.thecatapi.com/v1/images/search";
const DOG_URL: &str = "https://dog.ceo/api/breeds/image/random";
const UNKNOWN_IMAGE_URL: &str = "https://i.imgur.com/WQxgQUb.jpg";
const ANIMALS_CONTAINER: &str = "animalpics";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber)]
struct AnimalImageDownloaderActor {}

#[async_trait]
impl MessageSubscriber for AnimalImageDownloaderActor {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        let client = HttpClientSender::new();

        // Depending on the message, use public APIs to determine the image URL of an animal picture
        let api_response = client
            .request(ctx, &HttpRequest::get(&animal_req_url(&msg.subject)))
            .await?;
        let animal_image_url = parse_api_response(api_response);

        // Download the animal picture from the specified URL
        let img = client
            .request(ctx, &HttpRequest::get(&animal_image_url))
            .await?;

        let blobstore = BlobstoreSender::new();
        // Ensure the container is created to hold the picture
        if !blobstore
            .container_exists(ctx, &ANIMALS_CONTAINER.to_string())
            .await?
        {
            blobstore
                .create_container(ctx, &ANIMALS_CONTAINER.to_string())
                .await?
        }

        // Upload the picture to the blobstore
        blobstore
            .put_object(
                ctx,
                &PutObjectRequest {
                    chunk: Chunk {
                        container_id: ANIMALS_CONTAINER.to_string(),
                        object_id: "animal.png".to_string(),
                        bytes: img.body,
                        offset: 0,
                        is_last: true,
                    },
                    content_type: Some("image/png".to_string()),
                    ..Default::default()
                },
            )
            .await?;

        // Reply to the request noting that the animal picture is ready
        if let Some(reply_to) = msg.reply_to.as_ref() {
            MessagingSender::new()
                .publish(
                    ctx,
                    &PubMessage {
                        subject: reply_to.to_string(),
                        reply_to: None,
                        body: "Enjoy your animal picture 🐶🐱".to_string().into_bytes(),
                    },
                )
                .await?;
        }

        Ok(())
    }
}

/// Helper function to transform incoming message topic to a URL to request
fn animal_req_url(topic: &str) -> String {
    match topic.trim_start_matches(REQUEST_PREFIX) {
        "cat" => CAT_URL,
        "dog" => DOG_URL,
        _ => UNKNOWN_IMAGE_URL,
    }
    .to_string()
}

/// Helper function to parse thecatapi / dog.ceo API responses for the
/// embedded image_url
fn parse_api_response(api_response: HttpResponse) -> String {
    match serde_json::from_slice::<Value>(&api_response.body) {
        Ok(v) => {
            if let Value::String(image_url) = &v["message"] {
                image_url.to_string()
            } else if let Some(Value::String(image_url)) = v[0].get("url") {
                image_url.to_string()
            } else {
                UNKNOWN_IMAGE_URL.to_string()
            }
        }
        _ => UNKNOWN_IMAGE_URL.to_string(),
    }
}
