use crate::common::{get_memeber_user_id, get_option_value};
use lambda_runtime::Error;
use lib::deez::{DeezEntity, Deez};
use lib::discord::{Embed, Field, InteractionBody, ResponseData};
use lib::entity::prediction::Prediction;

pub async fn create(d: &Deez, b: &InteractionBody) -> Result<ResponseData, Error> {
    let condition = get_option_value(b, "namce")?.to_string()?;

    // todo: mnemonic prediction_id
    let prediction = Prediction {
        user_id: Some(get_memeber_user_id(b)?.to_string()),
        condition: Some(condition.to_string()),
        ..Prediction::generated_values()
    };

    d.put(&prediction).send().await?;

    Ok(ResponseData {
        embeds: Some(vec![Embed {
            title: Some(format!("New Prediction")),
            description: Some(condition.to_string()),
            fields: Some(vec![
                Field {
                    name: Some(format!("By")),
                    value: Some(format!("<@{}>", get_memeber_user_id(b)?)),
                    inline: Some(false),
                },
                Field {
                    name: Some(format!("ID")),
                    value: Some(format!(
                        "{}",
                        prediction.prediction_id.ok_or("missing prediction_id")?
                    )),
                    inline: Some(false),
                },
            ]),
            ..Default::default()
        }]),
        ..Default::default()
    })
}
