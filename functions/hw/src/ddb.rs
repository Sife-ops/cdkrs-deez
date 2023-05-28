use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

// todo: expose
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum IndexName {
    Primary,
    Gsi1,
    Gsi2,
}

#[derive(Debug)]
pub struct Composite {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Key {
    pub field: String,
    pub composite: Vec<Composite>,
}

impl Key {
    fn _join_composite(&self) -> String {
        let mut s = String::new();
        for composite in self.composite.iter() {
            s.push_str("#");
            s.push_str(&composite.name);
            s.push_str("_");
            s.push_str(&composite.value);
        }
        s
    }
}

#[derive(Debug)]
pub struct Index {
    pub partition_key: Key,
    pub sort_key: Key,
}

pub enum Attribute {
    DdbString(Option<String>),
    DdbBoolean(Option<bool>),
    DdbNumber(Option<i64>),
}

pub struct EntitySchema {
    pub table: String,
    pub service: String,
    pub entity: String,
    pub indices: HashMap<IndexName, Index>,
    pub attributes: HashMap<String, Attribute>,
}

pub trait DdbEntity {
    fn entity_schema(&self) -> EntitySchema;

    fn entity_to_av_map(&self) -> HashMap<String, AttributeValue> {
        let entity_schema = self.entity_schema();
        let mut m = HashMap::new();
        m.insert(
            "_entity".to_string(),
            AttributeValue::S(entity_schema.entity.clone()),
        );
        // indexes
        for (_, index) in &entity_schema.indices {
            // partition key
            let mut pk = String::from("$");
            pk.push_str(&entity_schema.service);
            pk.push_str("#");
            pk.push_str(&entity_schema.entity);
            pk.push_str(&index.partition_key._join_composite());
            m.insert(index.partition_key.field.clone(), AttributeValue::S(pk));
            // sort key
            let mut sk = String::from("$");
            sk.push_str(&entity_schema.entity);
            sk.push_str(&index.sort_key._join_composite());
            m.insert(index.sort_key.field.clone(), AttributeValue::S(sk));
        }
        // attributes
        for (k, v) in &entity_schema.attributes {
            match v {
                Attribute::DdbString(o) => {
                    if let Some(s) = o {
                        m.insert(k.to_string(), AttributeValue::S(s.to_string()));
                    }
                }
                Attribute::DdbBoolean(_) => {} // todo
                Attribute::DdbNumber(_) => {}  // todo
            };
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
    use crate::entity;

    #[test]
    fn av() {
        let p = entity::Prediction {
            prediction_id: "c".to_string(),
            user_id: "d".to_string(),
            condition: None,
            created_at: None,
        };

        let avm = p.entity_to_av_map();

        let pkv = avm.get("pk").unwrap().as_s().unwrap();
        // println!("{}", pkv);
        assert_eq!(pkv, "$Cdkrs#Prediction#predictionid_c");
    }
}
