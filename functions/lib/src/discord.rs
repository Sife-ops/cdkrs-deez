use crate::entity::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct InteractionBody {
    pub application_id: String,
    pub id: String,
    pub token: String,
    #[serde(rename(deserialize = "type"))]
    pub interaction_type: usize,
    pub version: usize,
    pub member: Option<Member>,        // todo: remove option
    pub data: Option<InteractionData>, // todo: remove option
}

#[derive(Debug, Deserialize, Clone)]
pub struct Member {
    pub user: User,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InteractionData {
    pub guild_id: String,
    pub id: String,
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub data_type: usize,
    pub options: Option<Vec<CommandOption>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommandOption {
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub option_type: usize,
    pub value: CommandOptionValue,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CommandOptionValue {
    STRING(String),
    USER(String),
    INTEGER(isize),
}

// #[derive(Debug, Clone)]
// pub struct CommandOptionValueError;
// impl std::fmt::Display for CommandOptionValueError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "unexpected value type")
//     }
// }
// impl std::error::Error for CommandOptionValueError {
//     // todo: definitions
// }

impl CommandOptionValue {
    // // todo: not a real "to_string" method, change name
    // pub fn to_string(&self) -> Result<&String, CommandOptionValueError> {
    //     if let Self::STRING(a) = self {
    //         return Ok(a);
    //     }
    //     Err(CommandOptionValueError)
    // }
    pub fn string(&self) -> Option<&String> {
        match self {
            Self::STRING(s) => Some(s),
            _ => None,
        }
    }
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
