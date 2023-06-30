mod commands;
mod common;

use aws_sdk_dynamodb::types::AttributeValue;
use deez::{create, vec_from_query};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use lib::discord::InteractionBody;
use lib::entity::user::{User, UserItems};
use lib::service::make_dynamo_client;
use std::collections::HashMap;
use std::env;
use ureq;

fn get_command_name(e: &InteractionBody) -> Result<&String, Error> {
    Ok(&e.data.as_ref().ok_or("missing command name")?.name)
}

async fn function_handler(event: LambdaEvent<InteractionBody>) -> Result<(), Error> {
    let dynamodb_client = &make_dynamo_client().await;

    // onboard user
    let user = &event.payload.member.as_ref().unwrap().user;
    let user_keys = user.primary_keys();
    let users = vec_from_query!(
        dynamodb_client
            .query()
            .table_name(User::table_name())
            .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), user_keys.hash.field()),
                ("#sk".to_string(), user_keys.range.field()),
            ])))
            .set_expression_attribute_values(Some(HashMap::from([
                (":pk".to_string(), user_keys.hash.av()),
                (":sk".to_string(), user_keys.range.av()),
            ])))
            .send()
            .await?
        => UserItems
    );
    if users.len() < 1 {
        create!(
            dynamodb_client;
            user.to_owned()
        )?;
    } else {
        let mut send_flag = false;
        let mut update_exp = "SET".to_string();
        let mut builder = dynamodb_client
            .update_item()
            .table_name(User::table_name())
            .set_key(Some(HashMap::from([
                (user_keys.hash.field(), user_keys.hash.av()),
                (user_keys.range.field(), user_keys.range.av()),
            ])));
        let first = users.first().unwrap();
        if first.globalname != user.globalname {
            send_flag = true;
            update_exp.push_str(" #globalname = :globalname");
            builder = builder
                .expression_attribute_names("#globalname".to_string(), "globalname".to_string())
                .expression_attribute_values(
                    ":globalname".to_string(),
                    AttributeValue::S(user.globalname.to_owned()),
                );
        }
        if first.username != user.username {
            send_flag = true;
            update_exp.push_str(" #username = :username");
            builder = builder
                .expression_attribute_names("#username".to_string(), "username".to_string())
                .expression_attribute_values(
                    ":username".to_string(),
                    AttributeValue::S(user.username.to_owned()),
                );
        }
        if first.avatar != user.avatar {
            send_flag = true;
            update_exp.push_str(" #avatar = :avatar");
            builder = builder
                .expression_attribute_names("#avatar".to_string(), "avatar".to_string())
                .expression_attribute_values(
                    ":avatar".to_string(),
                    AttributeValue::S(user.avatar.to_owned()),
                );
        }
        if send_flag {
            builder.update_expression(update_exp).send().await?;
        }
    }

    let res = match get_command_name(&event.payload)?.as_str() {
        "foo" => commands::foo::foo(&event.payload).await?,
        "create" => commands::create::create(dynamodb_client, &event.payload).await?,
        "vote" => commands::vote::vote(dynamodb_client, &event.payload).await?,
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
