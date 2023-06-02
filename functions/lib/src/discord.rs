use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionBody {
    application_id: String,
    id: String,
    token: String,
    #[serde(rename(deserialize = "type"))]
    interaction_type: usize,
    version: usize,
}
