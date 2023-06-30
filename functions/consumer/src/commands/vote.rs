use crate::common::{get_memeber_user_id, get_option_value};
use aws_sdk_dynamodb::Client;
use deez::{create, vec_from_query};
use lambda_runtime::Error;
use lib::discord::{Embed, Field, InteractionBody, ResponseData};
use lib::entity::prediction::{Prediction, PredictionItems};
use lib::entity::voter::{Voter, VoterItems};
use std::collections::HashMap;

pub async fn vote(
    client: &Client,
    interaction_body: &InteractionBody,
) -> Result<ResponseData, Error> {
    let user_id = get_memeber_user_id(interaction_body)?;
    let prediction_id = get_option_value(interaction_body, "id")?
        .string()
        .ok_or("unexpected value type")?;
    let vote = get_option_value(interaction_body, "vote")?
        .boolean()
        .ok_or("unexpected value type")?;

    let prediction_keys = Prediction {
        predictionid: Some(prediction_id.to_string()),
        ..Default::default()
    }
    .primary_keys();

    // exists
    let predictions = vec_from_query!(
        client
            .query()
            .table_name(Prediction::table_name())
            .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), prediction_keys.hash.field()),
                ("#sk".to_string(), prediction_keys.range.field()),
            ])))
            .set_expression_attribute_values(Some(HashMap::from([
                (":pk".to_string(), prediction_keys.hash.av()),
                (":sk".to_string(), prediction_keys.range.av()),
            ])))
            .send()
            .await?

        => PredictionItems
    );

    if predictions.len() < 1 {
        return Ok(ResponseData {
            content: Some(format!("<@{user_id}> voted on a nonexistent prediction.")),
            ..Default::default()
        });
    }

    // no self vote
    let prediction = predictions.first().unwrap();
    if prediction.userid.as_ref().unwrap() == &user_id {
        return Ok(ResponseData {
            content: Some(format!(
                "<@{user_id}> tried to vote on xis/xer own prediction."
            )),
            ..Default::default()
        });
    }

    // prevent double voting
    let voter_keys = Voter {
        userid: Some(user_id.to_string()),
        predictionid: Some(prediction_id.to_string()),
        ..Default::default()
    }
    .primary_keys();

    let voters = vec_from_query!(
        client
            .query()
            .table_name(Voter::table_name())
            .index_name(Voter::gsi1_name())
            .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), voter_keys.hash.field()),
                ("#sk".to_string(), voter_keys.range.field()),
            ])))
            .set_expression_attribute_values(Some(HashMap::from([
                (":pk".to_string(), voter_keys.hash.av()),
                (":sk".to_string(), voter_keys.range.av()),
            ])))
            .send()
            .await?

        => VoterItems
    );

    if voters.len() > 0 {
        return Ok(ResponseData {
            content: Some(format!("<@{user_id}> tried to vote twice.")),
            ..Default::default()
        });
    }

    // create vote
    create!(
        client;
        Voter {
            predictionid: Some(prediction_id.to_string()),
            userid: Some(user_id.to_string()),
            vote: *vote,
            ..Default::default()
        }
    )?;

    // respond
    let text: String;
    let color: usize;
    if *vote {
        text = format!("in favor of");
        color = 65280;
    } else {
        text = format!("against");
        color = 16711680;
    }

    Ok(ResponseData {
        content: Some(format!("voted")),
        embeds: Some(vec![Embed {
            title: Some(format!("Vote")),
            color: Some(color),
            description: Some(format!(
                "<@{}> voted {} <@{}>'s prediction:",
                user_id,
                text,
                prediction.userid.as_ref().unwrap()
            )),
            fields: Some(vec![
                Field {
                    name: Some(format!("Condition(s)")),
                    value: Some(prediction.condition.to_string()),
                    inline: Some(false),
                },
                Field {
                    name: Some(format!("ID")),
                    value: Some(prediction.predictionid.as_ref().unwrap().to_string()),
                    inline: Some(false),
                },
            ]),
            ..Default::default()
        }]),
        ..Default::default()
    })
}
