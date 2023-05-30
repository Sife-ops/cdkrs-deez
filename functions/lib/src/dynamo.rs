use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Key {
    pub field: String,
    pub composite: Vec<String>,
}

impl Key {
    fn _join_composite(&self, es: &EntitySchema) -> String {
        let mut c = String::new();
        for composite in self.composite.iter() {
            c.push_str(&format!(
                "#{}_{}",
                &composite,
                es.attributes.get(composite).unwrap().get_string()
            ));
        }
        c
    }
}

#[derive(Debug)]
pub struct Index {
    pub partition_key: Key,
    pub sort_key: Key,
}

#[derive(Debug)]
pub struct Value<T> {
    pub value: Option<T>,
    pub default: Option<T>,
}

impl<T: Clone> Value<T> {
    fn get(&self) -> Option<T> {
        if let Some(s) = self.value.clone() {
            return Some(s);
        } else if let Some(s) = self.default.clone() {
            return Some(s);
        }
        None
    }
}

#[derive(Debug)]
pub enum Attribute {
    DdbString(Value<String>),
    DdbNumber(Value<i64>),
    DdbBoolean(Value<bool>),
}

impl Attribute {
    fn get_string(&self) -> String {
        match self {
            Attribute::DdbString(y) => {
                return y.get().unwrap();
            }
            Attribute::DdbNumber(y) => {
                return y.get().unwrap().to_string();
            }
            Attribute::DdbBoolean(_) => {
                panic!("don't use boolean for id stupid");
            }
        }
    }
}

pub struct EntitySchema {
    pub table: String,
    pub service: String,
    pub entity: String,
    pub indices: HashMap<String, Index>,
    pub attributes: HashMap<String, Attribute>,
}

pub trait DdbEntity {
    fn entity_schema(&self) -> EntitySchema;

    fn entity_to_av_map(&self) -> HashMap<String, AttributeValue> {
        let entity_schema = self.entity_schema();
        let mut m = HashMap::new();
        m.insert(
            format!("_entity"),
            AttributeValue::S(entity_schema.entity.clone()),
        );
        // attributes
        for (name, attr) in &entity_schema.attributes {
            match attr {
                Attribute::DdbString(v) => {
                    if let Some(s) = v.get() {
                        m.insert(name.to_string(), AttributeValue::S(s));
                    }
                }
                Attribute::DdbBoolean(v) => {
                    if let Some(b) = v.get() {
                        m.insert(name.to_string(), AttributeValue::Bool(b));
                    }
                }
                Attribute::DdbNumber(v) => {
                    if let Some(b) = v.get() {
                        m.insert(name.to_string(), AttributeValue::N(b.to_string()));
                    }
                }
            };
        }
        // indexes
        for (_, index) in &entity_schema.indices {
            // partition key
            m.insert(
                index.partition_key.field.clone(),
                AttributeValue::S(format!(
                    "${}#{}{}",
                    &entity_schema.service,
                    &entity_schema.entity,
                    &index.partition_key._join_composite(&entity_schema)
                )),
            );
            // sort key
            m.insert(
                index.sort_key.field.clone(),
                AttributeValue::S(format!(
                    "${}{}",
                    &entity_schema.entity,
                    &index.sort_key._join_composite(&entity_schema)
                )),
            );
        }
        m
    }

    fn put(&self, c: &Client) -> PutItemFluentBuilder {
        let mut req = c.put_item().table_name(self.entity_schema().table);
        let m = self.entity_to_av_map();
        for (k, v) in &m {
            req = req.item(k, v.clone());
        }
        req
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::prediction::Prediction;

    #[test]
    fn av1() {
        let p1 = Prediction {
            prediction_id: Some(format!("a")),
            user_id: Some(format!("b")),
            ..Default::default()
        };

        let avm = p1.entity_to_av_map();
        println!("{:?}", avm);

        assert_eq!(
            avm.get("pk").unwrap().as_s().unwrap(),
            "$Cdkrs#Prediction#predictionid_a"
        );
        assert_eq!(avm.get("sk").unwrap().as_s().unwrap(), "$Prediction");
        assert_eq!(
            avm.get("gsi1pk").unwrap().as_s().unwrap(),
            "$Cdkrs#Prediction#userid_b"
        );
        assert_eq!(
            avm.get("gsi1sk").unwrap().as_s().unwrap(),
            "$Prediction#predictionid_a"
        );
        assert_eq!(
            avm.get("gsi2pk").unwrap().as_s().unwrap(),
            "$Cdkrs#Prediction#predictionid_a"
        );
        assert_eq!(avm.get("gsi2sk").unwrap().as_s().unwrap(), "$Prediction");
    }

    #[test]
    fn av2() {
        let p1 = Prediction {
            prediction_id: Some(format!("a")),
            user_id: Some(format!("b")),
            condition: Some(format!("c")),
            created_at: Some(format!("d")),
        };

        let avm = p1.entity_to_av_map();
        println!("{:?}", avm);

        assert_eq!(avm.get("predictionid").unwrap().as_s().unwrap(), "a");
        assert_eq!(avm.get("userid").unwrap().as_s().unwrap(), "b");
        assert_eq!(avm.get("condition").unwrap().as_s().unwrap(), "c");
        assert_eq!(avm.get("createdat").unwrap().as_s().unwrap(), "d");
    }
}
