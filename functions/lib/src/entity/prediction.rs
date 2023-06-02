use crate::deez::{Attribute, DeezEntity, EntityInfo, Index, Key};
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Utc;
use maplit::hashmap;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Prediction {
    pub prediction_id: Option<String>,
    pub user_id: Option<String>,
    pub condition: Option<String>,
    pub created_at: Option<String>,
}

// todo: https://users.rust-lang.org/t/access-struct-attributes-by-string/17520/2
impl DeezEntity for Prediction {
    fn info(&self) -> EntityInfo {
        EntityInfo {
            table: env::var("TABLE_NAME").unwrap_or(format!("MISSING")),
            service: format!("Cdkrs"),
            entity: format!("Prediction"),
        }
    }

    fn index_schema(&self) -> HashMap<String, Index> {
        hashmap! {
            format!("primary") => {
                Index {
                    partition_key: Key {
                        field: format!("pk"),
                        composite: vec![format!("predictionid")],
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
                        composite: vec![],
                    },
                }
            },
        }
    }

    fn attributes(&self) -> HashMap<String, Attribute> {
        hashmap! {
            format!("predictionid") => Attribute::DdbString(self.prediction_id.clone()),
            format!("userid") => Attribute::DdbString(self.user_id.clone()),
            format!("condition") => Attribute::DdbString(self.condition.clone()),
            format!("createdat") => Attribute::DdbString(self.created_at.clone()),
        }
    }

    fn generated_values() -> Self {
        Prediction {
            prediction_id: Some(Uuid::new_v4().to_string()),
            created_at: Some(Utc::now().to_rfc3339()),
            ..Default::default()
        }
    }

    fn from_map(m: &HashMap<String, AttributeValue>) -> Self {
        let mut d = Prediction {
            ..Default::default()
        };
        for (k, v) in m {
            match k.as_str() {
                "predictionid" => d.prediction_id = Some(v.as_s().unwrap().clone()),
                "userid" => d.user_id = Some(v.as_s().unwrap().clone()),
                "condition" => d.condition = Some(v.as_s().unwrap().clone()),
                "createdat" => d.created_at = Some(v.as_s().unwrap().clone()),
                &_ => {}
            }
        }
        d
    }
}
