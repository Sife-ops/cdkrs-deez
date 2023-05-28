mod ddb;
mod entity;

// use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
// use crate::ddb::DdbEntity;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");
    let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

    // let sc = aws_config::load_from_env().await;
    // let c = Client::new(&sc);

    // entity::Prediction {
    //     condition: "a".to_string(),
    //     created_at: "a".to_string(),
    //     prediction_id: "a".to_string(),
    //     user_id: "a".to_string(),
    // }
    // .put(&c)
    // .send()
    // .await?;

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
