use crate::deez::{Attribute, DeezEntity, EntityInfo, Index, Key};
use aws_sdk_dynamodb::types::AttributeValue;
use maplit::hashmap;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

#[derive(Default, Debug, Clone)]
pub struct Voter {
    pub voter_id: String,
    pub prediction_id: String,
    pub user_id: String,
    pub vote: bool,
}

// todo: https://users.rust-lang.org/t/access-struct-attributes-by-string/17520/2
impl DeezEntity for Voter {
    fn info(&self) -> EntityInfo {
        EntityInfo {
            table: env::var("TABLE_NAME").unwrap_or(format!("MISSING")),
            service: format!("Cdkrs"),
            entity: format!("Voter"),
        }
    }

    fn index_schema(&self) -> HashMap<String, Index> {
        hashmap! {
            format!("primary") => {
                Index {
                    partition_key: Key {
                        field: format!("pk"),
                        composite: vec![format!("voterid")],
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
                        composite: vec![format!("predictionid")],
                    },
                }
            },
            format!("gsi2") => {
                Index {
                    partition_key: Key {
                        field: format!("gsi2pk"),
                        composite: vec![format!("predictionid")],
                    },
                    sort_key: Key {
                        field: format!("gsi2sk"),
                        composite: vec![format!("userid")],
                    },
                }
            },
        }
    }

    fn attributes(&self) -> HashMap<String, Attribute> {
        hashmap! {
            format!("voterid") => Attribute::DeezString(self.voter_id.clone()),
            format!("predictionid") => Attribute::DeezString(self.prediction_id.clone()),
            format!("userid") => Attribute::DeezString(self.user_id.clone()),
            format!("vote") => Attribute::DeezBoolean(self.vote.clone()),
        }
    }

    fn generated_values() -> Self {
        Voter {
            voter_id: Uuid::new_v4().to_string(),
            ..Default::default()
        }
    }

    fn from_map(m: &HashMap<String, AttributeValue>) -> Self {
        let mut x = Voter {
            ..Default::default()
        };
        for (k, v) in m {
            match k.as_str() {
                "voterid" => x.voter_id = v.as_s().unwrap().clone(),
                "predictionid" => x.prediction_id = v.as_s().unwrap().clone(),
                "userid" => x.user_id = v.as_s().unwrap().clone(),
                "vote" => x.vote = v.as_bool().unwrap().clone(),
                &_ => {}
            }
        }
        x
    }
}
