use deez::*;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Deez)]
#[deez_schema(table = "prod-glsst-table", service = "Glsst", entity = "User")]
#[deez_schema(primary_hash = "pk", primary_range = "sk")]
#[deez_schema(gsi1_name = "gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
#[deez_schema(gsi2_name = "gsi2", gsi2_hash = "gsi2pk", gsi2_range = "gsi2sk")]
pub struct Voter {
    #[deez_primary(key = "hash")]
    pub voterid: Option<String>,
    #[deez_gsi1(key = "range")]
    #[deez_gsi2(key = "hash")]
    pub predictionid: Option<String>,
    #[deez_gsi1(key = "hash")]
    #[deez_gsi2(key = "range")]
    pub userid: Option<String>,
    pub vote: bool,
}

impl Default for Voter {
    fn default() -> Self {
        Voter {
            voterid: Some(Uuid::new_v4().to_string()),
            predictionid: None,
            userid: None,
            vote: false,
        }
    }
}
