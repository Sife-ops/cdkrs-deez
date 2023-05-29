use crate::dynamo::{Attribute, Composite, DdbEntity, EntitySchema, Index, IndexName, Key};
use maplit::hashmap;
use std::env;

#[derive(Default, Debug)]
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
            table: env::var("TABLE_NAME").unwrap_or(format!("MISSING")),
            service: format!("Cdkrs"),
            entity: format!("Prediction"),
            indices: hashmap! {
                IndexName::Primary => {
                    Index {
                        partition_key: Key {
                            field: format!("pk"),
                            composite: vec![Composite {
                                name: format!("predictionid"),
                                value: self.prediction_id.to_string(),
                            }],
                        },
                        sort_key: Key {
                            field: format!("sk"),
                            composite: vec![],
                        },
                    }
                },
                IndexName::Gsi1 => {
                    Index {
                        partition_key: Key {
                            field: format!("gsi1pk"),
                            composite: vec![Composite {
                                name: format!("userid"),
                                value: self.user_id.to_string(),
                            }],
                        },
                        sort_key: Key {
                            field: format!("gsi1sk"),
                            composite: vec![Composite {
                                name: format!("predictionid"),
                                value: self.prediction_id.to_string(),
                            }],
                        },
                    }
                },
                IndexName::Gsi2 => {
                    Index {
                        partition_key: Key {
                            field: format!("gsi2pk"),
                            composite: vec![Composite {
                                name: format!("predictionid"),
                                value: self.prediction_id.to_string(),
                            }],
                        },
                        sort_key: Key {
                            field: format!("gsi2sk"),
                            composite: vec![],
                        },
                    }
                },
            },
            attributes: hashmap! {
                format!("predictionid") => Attribute::DdbString(Some(self.prediction_id.clone())),
                format!("userid") => Attribute::DdbString(Some(self.user_id.clone())),
                format!("condition") => Attribute::DdbString(self.condition.clone()),
                format!("createdat") => Attribute::DdbString(self.created_at.clone()),
            },
        }
    }
}
