mod commands;
mod common;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use lib::deez::Deez;
use lib::discord::InteractionBody;
use lib::onboard::onboard;
use lib::service::make_dynamo_client;
use std::env;
use ureq;

fn get_command_name(e: &InteractionBody) -> Result<&String, Error> {
    Ok(&e.data.as_ref().ok_or("missing command name")?.name)
}

async fn function_handler(event: LambdaEvent<InteractionBody>) -> Result<(), Error> {
    let deez = &Deez::new(make_dynamo_client().await);
    onboard(deez, common::get_member_user(&event.payload)?).await;

    let res = match get_command_name(&event.payload)?.as_str() {
        "foo" => commands::foo::foo(&event.payload).await?,
        "create" => commands::create::create(deez, &event.payload).await?,
        &_ => panic!("unknown command name"),
    };

    let agent = ureq::agent();
    agent
        .post(&format!(
            "https://discord.com/api/v10/webhooks/{}/{}",
            env::var("BOT_APP_ID")?,
            event.payload.token
        ))
        .set("Content-Type", "application/json")
        .set("Authorization", &env::var("BOT_PUBLIC_KEY")?)
        .send_string(&serde_json::to_string(&res)?)?
        .into_string()?;

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
