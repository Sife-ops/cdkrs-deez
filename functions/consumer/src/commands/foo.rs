use lib::discord::{InteractionBody, ResponseData};

pub async fn foo(_b: &InteractionBody) -> ResponseData {
    ResponseData {
        content: Some(format!("bar")),
        ..Default::default()
    }
}
