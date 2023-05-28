use crate::ddb::{Attribute, Composite, DdbEntity, EntitySchema, Index, IndexName, Key};
use std::collections::HashMap;

pub struct Prediction {
    pub prediction_id: String,
    pub user_id: String,
    pub condition: String,
    pub created_at: String,
}

// todo: https://users.rust-lang.org/t/access-struct-attributes-by-string/17520/2
impl DdbEntity for Prediction {
    fn entity_schema(&self) -> EntitySchema {
        let mut indices = HashMap::new();
        indices.insert(
            IndexName::Primary,
            Index {
                partition_key: Key {
                    field: "pk".to_string(),
                    composite: vec![Composite {
                        name: "prediction_id".to_string(),
                        value: self.prediction_id.to_string(),
                    }],
                },
                sort_key: Key {
                    field: "sk".to_string(),
                    composite: vec![],
                },
            },
        );

        let mut attributes = HashMap::new();
        attributes.insert(
            "prediction_id".to_string(),
            Attribute::DdbString(self.prediction_id.to_string()),
        );
        attributes.insert(
            "user_id".to_string(),
            Attribute::DdbString(self.user_id.to_string()),
        );
        attributes.insert(
            "condition".to_string(),
            Attribute::DdbString(self.condition.to_string()),
        );
        attributes.insert(
            "created_at".to_string(),
            Attribute::DdbString(self.created_at.to_string()), // todo: time type
        );

        EntitySchema {
            service: "Cdkrs".to_string(),
            entity: "Prediction".to_string(),
            indices, // todo: hashlit?
            attributes,
        }
    }
}
