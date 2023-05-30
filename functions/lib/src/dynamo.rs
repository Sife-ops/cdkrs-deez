use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Key {
    pub field: String,
    pub composite: Vec<String>,
}

impl Key {
    // todo: prop-drill shittiness
    fn _join_composite(&self, attrs: &HashMap<String, Attribute>, default: &DefaultAttr) -> String {
        let mut c = String::new();
        for composite in self.composite.iter() {
            c.push_str(&format!(
                "#{}_{}",
                &composite,
                attrs.get(composite).unwrap().get_string(default)
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
    pub required: bool,
    pub default: Option<T>, // todo: use closures
}

#[derive(Debug)]
pub enum Attribute {
    DdbString(Value<String>),
    DdbNumber(Value<i64>),
    DdbBoolean(Value<bool>),
}

#[derive(PartialEq)]
pub enum DefaultAttr {
    Use,
    Ignore,
}

impl<T: Clone> Value<T> {
    fn get(&self, default: &DefaultAttr) -> Option<T> {
        if let Some(s) = self.value.clone() {
            return Some(s);
        } else if self.required {
            return None;
        } else if default == &DefaultAttr::Ignore {
            return None;
        } else if let Some(s) = self.default.clone() {
            return Some(s);
        }
        None
    }
}

impl Attribute {
    fn get_string(&self, default: &DefaultAttr) -> String {
        match self {
            Attribute::DdbString(y) => {
                return y.get(default).unwrap();
            }
            Attribute::DdbNumber(y) => {
                return y.get(default).unwrap().to_string();
            }
            Attribute::DdbBoolean(_) => {
                panic!("don't use boolean for id stupid");
            }
        }
    }
}

pub struct EntityInfo {
    pub table: String,
    pub service: String,
    pub entity: String,
}

pub trait DdbEntity {
    fn info(&self) -> EntityInfo;

    fn index_schema(&self) -> HashMap<String, Index>;

    fn attributes(&self) -> HashMap<String, Attribute>;

    fn entity_to_av_map(&self, default: &DefaultAttr) -> HashMap<String, AttributeValue> {
        let info = self.info();
        let mut m = HashMap::new();
        m.insert(format!("_entity"), AttributeValue::S(info.entity.clone()));

        // attributes
        let attrs = self.attributes();
        for (name, attr) in &attrs {
            match attr {
                Attribute::DdbString(v) => {
                    if let Some(s) = v.get(default) {
                        m.insert(name.to_string(), AttributeValue::S(s));
                    }
                }
                Attribute::DdbBoolean(v) => {
                    if let Some(s) = v.get(default) {
                        m.insert(name.to_string(), AttributeValue::Bool(s));
                    }
                }
                Attribute::DdbNumber(v) => {
                    if let Some(s) = v.get(default) {
                        m.insert(name.to_string(), AttributeValue::N(s.to_string()));
                    }
                }
            };
        }

        // indexes
        let is = self.index_schema();
        for (_, index) in &is {
            // partition key
            m.insert(
                index.partition_key.field.clone(),
                AttributeValue::S(format!(
                    "${}#{}{}",
                    info.service,
                    info.entity,
                    index.partition_key._join_composite(&attrs, &DefaultAttr::Use),
                )),
            );
            // sort key
            m.insert(
                index.sort_key.field.clone(),
                AttributeValue::S(format!(
                    "${}{}",
                    info.entity,
                    index.sort_key._join_composite(&attrs, &DefaultAttr::Use)
                )),
            );
        }
        m
    }

    fn put(&self, c: &Client) -> PutItemFluentBuilder {
        let mut req = c.put_item().table_name(self.info().table);
        let m = self.entity_to_av_map(&DefaultAttr::Use);
        for (k, v) in &m {
            req = req.item(k, v.clone());
        }
        req
    }

    // fn q(&self, c: &Client) -> bool {
    //     let mut req = c.query().table_name(self.table_name());
    //     let a = self.entity_schema();
    //     let b = self.entity_to_av_map();
    //     true
    // }
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

        let avm = p1.entity_to_av_map(&DefaultAttr::Use);
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

        let avm = p1.entity_to_av_map(&DefaultAttr::Use);
        println!("{:?}", avm);

        assert_eq!(avm.get("predictionid").unwrap().as_s().unwrap(), "a");
        assert_eq!(avm.get("userid").unwrap().as_s().unwrap(), "b");
        assert_eq!(avm.get("condition").unwrap().as_s().unwrap(), "c");
        assert_eq!(avm.get("createdat").unwrap().as_s().unwrap(), "d");
    }

    #[test]
    fn av3() {
        let p1 = Prediction {
            user_id: Some(format!("b")),
            condition: Some(format!("c")),
            created_at: Some(format!("d")),
            ..Default::default()
        };

        let avm = p1.entity_to_av_map(&DefaultAttr::Use);
        println!("=================");
        println!("{:?}", avm);

        assert_eq!(1, 1);
    }
}
