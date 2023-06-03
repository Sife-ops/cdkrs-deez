use crate::common::{get_memeber_user_id, get_option_value};
use lambda_runtime::Error;
use lib::deez::{Deez, DeezEntity, Index};
use lib::discord::{Embed, Field, InteractionBody, ResponseData};
use lib::entity::indexes;
use lib::entity::prediction::Prediction;
use lib::entity::voter::Voter;

pub async fn vote(d: &Deez, b: &InteractionBody) -> Result<ResponseData, Error> {
    let user_id = get_memeber_user_id(b)?;
    let prediction_id = get_option_value(b, "id")?
        .string()
        .ok_or("unexpected value type")?;
    let vote = get_option_value(b, "vote")?
        .boolean()
        .ok_or("unexpected value type")?;

    // prediction must exist
    let predictions_query = d
        .query(
            Index::Primary,
            &Prediction {
                prediction_id: prediction_id.to_string(),
                ..Default::default()
            },
        )
        .send()
        .await?;
    if predictions_query
        .items()
        .ok_or("missing predictions slice")?
        .len()
        < 1
    {
        return Ok(ResponseData {
            content: Some(format!("<@{user_id}> voted on a nonexistent prediction.")),
            ..Default::default()
        });
    }

    // no self vote
    let predictions = Prediction::from_map_slice(predictions_query.items().unwrap());
    let prediction = predictions.first().unwrap();
    if prediction.user_id == *user_id {
        // todo: sussy
        return Ok(ResponseData {
            content: Some(format!(
                "<@{user_id}> tried to vote on xis/xer own prediction."
            )),
            ..Default::default()
        });
    }

    // prevent double voting
    let voters_query = d
        .query(
            indexes::GSI1,
            &Voter {
                user_id: user_id.to_string(),
                prediction_id: prediction_id.to_string(),
                ..Default::default()
            },
        )
        .send()
        .await?;
    if voters_query.items().unwrap().len() > 0 {
        return Ok(ResponseData {
            content: Some(format!("<@{user_id}> tried to vote twice.")),
            ..Default::default()
        });
    }

    // insert vote
    d.put(&Voter {
        prediction_id: prediction_id.to_string(),
        user_id: user_id.to_string(),
        vote: *vote,
        ..Voter::generated_values()
    })
    .send()
    .await?;

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
                user_id, text, prediction.user_id
            )),
            fields: Some(vec![
                Field {
                    name: Some(format!("Condition(s)")),
                    value: Some(prediction.condition.to_string()),
                    inline: Some(false),
                },
                Field {
                    name: Some(format!("ID")),
                    value: Some(prediction.prediction_id.to_string()),
                    inline: Some(false),
                },
            ]),
            ..Default::default()
        }]),
        ..Default::default()
    })
}
