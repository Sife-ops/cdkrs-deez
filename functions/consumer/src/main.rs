mod commands;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use lib::discord::InteractionBody;
use std::env;
use ureq;

async fn function_handler(event: LambdaEvent<InteractionBody>) -> Result<(), Error> {
    // todo: onboard user

    let res = match event.payload.data.as_ref().unwrap().name.as_str() {
        "foo" => commands::foo::foo(&event.payload).await,
        &_ => panic!("unknown command name"),
    };

    let a = ureq::agent();
    let b = a
        .post(&format!(
            "https://discord.com/api/v10/webhooks/{}/{}",
            env::var("BOT_APP_ID")?,
            event.payload.token
        ))
        .set("Content-Type", "application/json")
        .set("Authorization", &env::var("BOT_PUBLIC_KEY")?)
        .send_string(&serde_json::to_string(&res)?)?
        .into_string()?;
    println!("{}", b);

    Ok(())
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
