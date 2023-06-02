use crate::entity::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct InteractionBody {
    pub application_id: String,
    pub id: String,
    pub token: String,
    #[serde(rename(deserialize = "type"))]
    pub interaction_type: usize,
    pub version: usize,
    pub member: Option<Member>, // todo: remove option
    pub data: Option<InteractionData>, // todo: remove option
}

#[derive(Debug, Deserialize)]
pub struct Member {
    pub user: Option<User>,
}

#[derive(Debug, Deserialize)]
pub struct InteractionData {
    pub guild_id: String,
    pub id: String,
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub data_type: usize,
    pub options: Option<Vec<CommandOption>>,
}

#[derive(Debug, Deserialize)]
pub struct CommandOption {
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub option_type: usize,
    // todo: value
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Debug, Default)]
pub struct ResponseBody {
    #[serde(rename(serialize = "type"))]
    pub response_type: usize,
    pub data: Option<ResponseData>,
}

#[derive(Serialize, Debug, Default)]
pub struct ResponseData {
    pub flags: Option<usize>,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>,
}

#[derive(Serialize, Debug, Default)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<usize>,
    pub url: Option<String>,
    pub fields: Option<Vec<Field>>,
}

#[derive(Serialize, Debug, Default)]
pub struct Field {
    pub name: Option<String>,
    pub value: Option<String>,
    pub inline: Option<bool>,
}
