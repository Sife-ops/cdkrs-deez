use crate::dynamo::{Attribute, DdbEntity, EntityInfo, Index, Key, Value};
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
impl DdbEntity for Prediction {
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
            format!("predictionid") => Attribute::DdbString(Value {
                value: self.prediction_id.clone(),
                required: true,
                default: Some(Uuid::new_v4().to_string()), // todo: don't generate default during query...
            }),
            format!("userid") => Attribute::DdbString(Value {
                value: self.user_id.clone(),
                required: true,
                default: None,
            }),
            format!("condition") => Attribute::DdbString(Value {
                value: self.condition.clone(),
                required: false,
                default: None,
            }),
            format!("createdat") => Attribute::DdbString(Value {
                value: self.created_at.clone(),
                required: false,
                default: None, // todo: rfc3339
            }),
        }
    }
}
