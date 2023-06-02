use ed25519_dalek::{PublicKey, Signature, Verifier};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct InteractionBody {
    application_id: String,
    id: String,
    token: String,
    #[serde(rename(deserialize = "type"))]
    interaction_type: usize,
    version: usize,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let body_text = match event.body() {
        Body::Text(t) => t,
        _ => panic!(), // return error?
    };

    // verification
    let interaction_body: InteractionBody = serde_json::from_str(&body_text)?;
    if interaction_body.interaction_type == 1 {
        // public key
        let public_key_string = env::var("BOT_PUBLIC_KEY")?;
        let public_key_vector = hex::decode(public_key_string)?;
        let public_key = PublicKey::from_bytes(&public_key_vector)?;

        // ed25519
        let signature_string = event
            .headers()
            .get("x-signature-ed25519")
            .unwrap()
            .to_str()?;
        let signature_vector = hex::decode(signature_string)?;
        let signature = Signature::try_from(&signature_vector[..])?;

        // timestamp
        let timestamp = event
            .headers()
            .get("x-signature-timestamp")
            .unwrap()
            .to_str()?;

        // verify
        let message = format!("{timestamp}{body_text}");
        return match public_key.verify(message.as_bytes(), &signature) {
            Ok(_) => Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(body_text.clone().into())
                .map_err(Box::new)?),
            Err(_) => Ok(Response::builder()
                .status(401)
                .body(Body::Empty)
                .map_err(Box::new)?
            ),
        };
    }

    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
