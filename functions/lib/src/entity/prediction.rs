use crate::deez::{Attribute, DeezEntity, EntityInfo, Index, IndexSchema, Key};
use crate::entity::indexes;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::Utc;
use fake::faker::company::en;
use fake::Fake;
use maplit::hashmap;
use std::collections::HashMap;
use std::env;

#[derive(Default, Debug)]
pub struct Prediction {
    pub prediction_id: String,
    pub user_id: String,
    pub condition: String,
    pub created_at: String,
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

    fn index_schemas(&self) -> HashMap<Index, IndexSchema> {
        hashmap! {
            Index::Primary => {
                IndexSchema {
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
            indexes::GSI1 => {
                IndexSchema {
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
            indexes::GSI2 => {
                IndexSchema {
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
            format!("predictionid") => Attribute::DeezString(self.prediction_id.clone()),
            format!("userid") => Attribute::DeezString(self.user_id.clone()),
            format!("condition") => Attribute::DeezString(self.condition.clone()),
            format!("createdat") => Attribute::DeezString(self.created_at.clone()),
        }
    }

    fn generated_values() -> Self {
        Prediction {
            prediction_id: format!(
                "{}-{}-{}",
                en::BsAdj().fake::<String>(),
                en::BsAdj().fake::<String>(),
                en::BsNoun().fake::<String>()
            ),
            created_at: Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }

    fn from_map(m: &HashMap<String, AttributeValue>) -> Self {
        let mut d = Prediction {
            ..Default::default()
        };
        for (k, v) in m {
            match k.as_str() {
                "predictionid" => d.prediction_id = v.as_s().unwrap().clone(),
                "userid" => d.user_id = v.as_s().unwrap().clone(),
                "condition" => d.condition = v.as_s().unwrap().clone(),
                "createdat" => d.created_at = v.as_s().unwrap().clone(),
                &_ => {}
            }
        }
        d
    }
}
