use lambda_runtime::Error;
use lib::discord::{InteractionBody, ResponseData};

pub async fn foo(_b: &InteractionBody) -> Result<ResponseData, Error> {
    Ok(ResponseData {
        content: Some(format!("bar")),
        ..Default::default()
    })
}
