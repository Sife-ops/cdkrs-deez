use aws_sdk_dynamodb::types::AttributeValue;
use deez::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Default, Debug, Deserialize, Clone, Deez)]
#[deez_schema(table = "prod-glsst-table", service = "Glsst", entity = "User")]
#[deez_schema(primary_hash = "pk", primary_range = "sk")]
#[deez_schema(gsi1_name = "gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
pub struct User {
    #[serde(rename(deserialize = "id"))]
    #[deez_primary(key = "hash")]
    #[deez_gsi1(key = "hash")]
    pub userid: Option<String>,
    pub username: String,
    pub discriminator: String,
    #[serde(rename(deserialize = "global_name"))]
    pub globalname: String,
    pub avatar: String,
}
