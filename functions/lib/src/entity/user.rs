use crate::deez::{Attribute, DeezEntity, EntityInfo, Index, IndexSchema, Key};
use crate::entity::indexes;
use aws_sdk_dynamodb::types::AttributeValue;
use maplit::hashmap;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Default, Debug, Deserialize, Clone)]
pub struct User {
    #[serde(rename(deserialize = "id"))]
    pub user_id: String,
    pub username: String,
    pub discriminator: String,
    pub display_name: Option<String>,
    pub global_name: Option<String>,
    pub avatar: String,
}

// todo: https://users.rust-lang.org/t/access-struct-attributes-by-string/17520/2
impl DeezEntity for User {
    fn info(&self) -> EntityInfo {
        EntityInfo {
            table: env::var("TABLE_NAME").unwrap_or(format!("MISSING")),
            service: format!("Glsst"),
            entity: format!("User"),
        }
    }

    fn index_schemas(&self) -> HashMap<Index, IndexSchema> {
        hashmap! {
            indexes::PRIMARY => {
                IndexSchema {
                    partition_key: Key {
                        field: format!("pk"),
                        composite: vec![format!("userid")],
                    },
                    sort_key: Key {
                        field: format!("sk"),
                        composite: vec![],
                    },
                }
            },
            indexes::GSI1 => {
                IndexSchema {
                    partition_key: Key {
                        field: format!("gsi1pk"),
                        composite: vec![format!("userid")],
                    },
                    sort_key: Key {
                        field: format!("gsi1sk"),
                        composite: vec![],
                    },
                }
            },
        }
    }

    fn attributes(&self) -> HashMap<String, Attribute> {
        hashmap! {
            format!("userid") => Attribute::DeezString(self.user_id.clone()),
            format!("username") => Attribute::DeezString(self.username.clone()),
            format!("discriminator") => Attribute::DeezString(self.discriminator.clone()),
            format!("displayname") => Attribute::DeezString(self.display_name.as_ref().unwrap_or(&format!("")).to_string()),
            format!("globalname") => Attribute::DeezString(self.global_name.as_ref().unwrap_or(&format!("")).to_string()),
            format!("avatar") => Attribute::DeezString(self.avatar.clone()),
        }
    }

    fn generated_values() -> Self {
        User {
            ..Default::default()
        }
    }

    fn from_map(m: &HashMap<String, AttributeValue>) -> Self {
        let mut d = User {
            ..Default::default()
        };
        for (k, v) in m {
            match k.as_str() {
                "userid" => d.user_id = v.as_s().unwrap().clone(),
                "username" => d.username = v.as_s().unwrap().clone(),
                "discriminator" => d.discriminator = v.as_s().unwrap().clone(),
                "displayname" => d.display_name = Some(v.as_s().unwrap().clone()),
                "globalname" => d.global_name = Some(v.as_s().unwrap().clone()),
                "avatar" => d.avatar = v.as_s().unwrap().clone(),
                &_ => {}
            }
        }
        d
    }
}
