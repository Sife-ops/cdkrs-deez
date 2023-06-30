use crate::common::{get_memeber_user_id, get_option_value};
use aws_sdk_dynamodb::Client;
use deez::{create, vec_from_query};
use lambda_runtime::Error;
use lib::discord::{Embed, Field, InteractionBody, ResponseData};
use lib::entity::prediction::{Prediction, PredictionItems};
use std::collections::HashMap;

pub async fn create(
    client: &Client,
    interaction_body: &InteractionBody,
) -> Result<ResponseData, Error> {
    let condition = get_option_value(interaction_body, "condition")?
        .string()
        .ok_or("unexpected value type")?;

    let mut prediction: Prediction;
    loop {
        prediction = Prediction {
            userid: Some(get_memeber_user_id(interaction_body)?.to_string()),
            condition: condition.to_string(),
            ..Default::default()
        };

        let prediction_keys = prediction.primary_keys();
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
            break;
        }
    }

    create!(
        client;
        prediction.clone()
    )?;

    Ok(ResponseData {
        embeds: Some(vec![Embed {
            title: Some(format!("New Prediction")),
            description: Some(condition.to_string()),
            fields: Some(vec![
                Field {
                    name: Some(format!("By")),
                    value: Some(format!("<@{}>", get_memeber_user_id(interaction_body)?)),
                    inline: Some(false),
                },
                Field {
                    name: Some(format!("ID")),
                    value: Some(format!("{}", prediction.predictionid.unwrap())),
                    inline: Some(false),
                },
            ]),
            ..Default::default()
        }]),
        ..Default::default()
    })
}
