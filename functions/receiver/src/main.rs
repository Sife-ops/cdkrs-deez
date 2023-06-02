use aws_sdk_lambda::types::InvocationType;
use aws_smithy_types::Blob;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lib::discord::{InteractionBody, ResponseBody};
use lib::service::make_lambda_client;
use serde_json;
use std::env;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let body_text = match event.body() {
        Body::Text(t) => t,
        _ => panic!(), // return error?
    };
    let interaction_body: InteractionBody = serde_json::from_str(&body_text)?;

    // verification
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
                .map_err(Box::new)?),
        };
    }

    let lambda_client = make_lambda_client().await;
    lambda_client
        .invoke()
        .function_name(env::var("CONSUMERFN_ARN")?)
        .invocation_type(InvocationType::Event)
        .payload(Blob::new(body_text.as_bytes()))
        .send()
        .await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&ResponseBody {
                response_type: 5,
                ..Default::default()
            })?
            .into(),
        )
        .map_err(Box::new)?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
