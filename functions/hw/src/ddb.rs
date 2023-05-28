use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum IndexName {
    Primary,
    Gsi1,
    Gsi2,
}

pub struct Composite {
    pub name: String,
    pub value: String,
}

pub struct Key {
    pub field: String,
    pub composite: Vec<Composite>,
}

pub struct Index {
    pub partition_key: Key,
    pub sort_key: Key,
}

pub enum Attribute {
    DdbString(String),
    DdbBoolean(bool),
    DdbNumber(i64),
}

pub struct EntitySchema {
    pub service: String,
    pub entity: String,
    pub indices: HashMap<IndexName, Index>,
    pub attributes: HashMap<String, Attribute>, // todo: generic value
}

fn key_to_av(s: &mut String, k: &Key) -> AttributeValue {
    for composite in k.composite.iter() {
        s.push_str("#");
        s.push_str(&composite.name);
        s.push_str("_");
        s.push_str(&composite.value);
    }
    AttributeValue::S(s.to_string())
}

pub trait DdbEntity {
    fn entity_schema(&self) -> EntitySchema;

    fn entity_to_av_map(&self) -> HashMap<String, AttributeValue> {
        let entity_schema = self.entity_schema();
        let mut m = HashMap::new();
        m.insert(
            "_entity".to_string(),
            AttributeValue::S(entity_schema.entity),
        );
        // indexes
        for (_, index) in &entity_schema.indices {
            // partition key
            let mut pk = String::from("$"); // todo: not dry
            pk.push_str(&entity_schema.service);
            m.insert(
                index.partition_key.field.clone(),
                key_to_av(&mut pk, &index.partition_key),
            );
            // sort key
            let mut sk = String::from("$");
            sk.push_str(&entity_schema.service);
            m.insert(
                index.sort_key.field.clone(),
                key_to_av(&mut sk, &index.sort_key),
            );
        }
        // attributes
        for (k, v) in &entity_schema.attributes {
            match v {
                Attribute::DdbString(a) => {
                    m.insert(k.to_string(), AttributeValue::S(a.to_string()));
                }
                Attribute::DdbBoolean(_) => {} // todo
                Attribute::DdbNumber(_) => {}  // todo
            };
        }
        m
    }

    fn put(&self, c: &Client) -> PutItemFluentBuilder {
        let mut req = c.put_item().table_name("cdkrs-table"); // todo: global
        let m = self.entity_to_av_map();
        for (k, v) in &m {
            req = req.item(k, v.clone());
        }
        req
    }
}

