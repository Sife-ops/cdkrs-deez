// use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use lib::dynamo::DdbEntity;
use lib::dynamo::Deez;
use lib::entity::prediction::Prediction;
use lib::service::make_dynamo_client;

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

    let d = Deez::new(make_dynamo_client().await);
    d.put(&Prediction {
        user_id: Some(format!("deez")),
        ..Prediction::generated_values()
    })
    .send()
    .await?;

    let q = d
        .query(
            "primary",
            &Prediction {
                prediction_id: Some(format!("41e3cdcb-1556-4a3c-a007-19ceb552b188")),
                ..Default::default()
            },
        )
        .send()
        .await?;
    let _p = Prediction::from_map_slice(q.items().unwrap());

    let message = format!("Hello {who}, this is an AWS Lambda HTTP request.");

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
