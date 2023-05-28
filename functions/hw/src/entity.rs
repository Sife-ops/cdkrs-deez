use crate::ddb::{Attribute, Composite, DdbEntity, EntitySchema, Index, IndexName, Key};
use maplit::hashmap;

pub struct Prediction {
    pub prediction_id: String,
    pub user_id: String,
    pub condition: Option<String>,
    pub created_at: Option<String>,
}

// todo: https://users.rust-lang.org/t/access-struct-attributes-by-string/17520/2
impl DdbEntity for Prediction {
    fn entity_schema(&self) -> EntitySchema {
        EntitySchema {
            table: "cdkrs-table".to_string(), // todo: read from env
            service: "Cdkrs".to_string(),
            entity: "Prediction".to_string(),
            indices: hashmap! {
                IndexName::Primary => {
                    Index {
                        partition_key: Key {
                            field: "pk".to_string(),
                            composite: vec![Composite {
                                name: "predictionid".to_string(),
                                value: self.prediction_id.to_string(),
                            }],
                        },
                        sort_key: Key {
                            field: "sk".to_string(),
                            composite: vec![],
                        },
                    }
                }
            },
            attributes: hashmap! {
                "predictionid".to_string() => Attribute::DdbString(Some(self.prediction_id.clone())),
                "userid".to_string() => Attribute::DdbString(Some(self.user_id.clone())),
                "condition".to_string() => Attribute::DdbString(self.condition.clone()),
                "createdat".to_string() => Attribute::DdbString(self.created_at.clone()),
            },
        }
    }
}
