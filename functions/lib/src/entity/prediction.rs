use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Utc;
use deez::*;
use fake::faker::company::en;
use fake::Fake;
use std::collections::HashMap;

#[derive(Debug, Deez, Clone)]
#[deez_schema(table = "prod-glsst-table", service = "Glsst", entity = "User")]
#[deez_schema(primary_hash = "pk", primary_range = "sk")]
#[deez_schema(gsi1_name = "gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
#[deez_schema(gsi2_name = "gsi2", gsi2_hash = "gsi2pk", gsi2_range = "gsi2sk")]
pub struct Prediction {
    #[deez_primary(key = "hash")]
    #[deez_gsi1(key = "range")]
    pub predictionid: Option<String>,
    #[deez_gsi1(key = "hash")]
    #[deez_gsi2(key = "hash")]
    pub userid: Option<String>,
    pub condition: String,
    pub createdat: String,
}

impl Default for Prediction {
    fn default() -> Self {
        Prediction {
            predictionid: Some(format!(
                "{}-{}-{}",
                en::BsAdj().fake::<String>(),
                en::BsAdj().fake::<String>(),
                en::BsNoun().fake::<String>()
            )),
            createdat: Utc::now().to_rfc3339(),
            condition: "".to_string(),
            userid: Some("".to_string()),
        }
    }
}
