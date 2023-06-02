use crate::dynamo::{Attribute, DdbEntity, EntityInfo, Index, Key};
use aws_sdk_dynamodb::types::AttributeValue;
use maplit::hashmap;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Default, Debug, Deserialize)]
pub struct User {
    #[serde(rename(deserialize = "id"))]
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub discriminator: Option<String>,
    pub display_name: Option<String>,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
}

// todo: https://users.rust-lang.org/t/access-struct-attributes-by-string/17520/2
impl DdbEntity for User {
    fn info(&self) -> EntityInfo {
        EntityInfo {
            table: env::var("TABLE_NAME").unwrap_or(format!("MISSING")),
            service: format!("Cdkrs"),
            entity: format!("User"),
        }
    }

    fn index_schema(&self) -> HashMap<String, Index> {
        hashmap! {
            format!("primary") => {
                Index {
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
            format!("gsi1") => {
                Index {
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
            format!("userid") => Attribute::DdbString(self.user_id.clone()),
            format!("username") => Attribute::DdbString(self.username.clone()),
            format!("discriminator") => Attribute::DdbString(self.discriminator.clone()),
            format!("displayname") => Attribute::DdbString(self.display_name.clone()),
            format!("globalname") => Attribute::DdbString(self.global_name.clone()),
            format!("avatar") => Attribute::DdbString(self.avatar.clone()),
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
                "userid" => d.user_id = Some(v.as_s().unwrap().clone()),
                "username" => d.username = Some(v.as_s().unwrap().clone()),
                "discriminator" => d.discriminator = Some(v.as_s().unwrap().clone()),
                "displayname" => d.display_name = Some(v.as_s().unwrap().clone()),
                "globalname" => d.global_name = Some(v.as_s().unwrap().clone()),
                "avatar" => d.avatar = Some(v.as_s().unwrap().clone()),
                &_ => {}
            }
        }
        d
    }
}
