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
    fn join_composite(&self, attrs: &HashMap<String, Attribute>, default: &DefaultAttr) -> String {
        let mut c = String::new();
        for composite in self.composite.iter() {
            let attr = attrs.get(composite).unwrap();
            if let Some(s) = attr.get_string(default) {
                c.push_str(&format!("#{}_{}", composite, s,));
            }
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
    pub default: Option<T>, // todo: closure
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
        if self.value.is_some() {
            return self.value.clone();
        }
        if self.default.is_some() && default == &DefaultAttr::Use {
            return self.default.clone();
        }
        None
    }
}

impl Attribute {
    fn get_string(&self, default: &DefaultAttr) -> Option<String> {
        match self {
            Attribute::DdbString(y) => {
                return y.get(default);
            }
            Attribute::DdbNumber(y) => {
                let x = y.get(default)?;
                return Some(x.to_string());
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

    // todo: eliminate prop-drill shittiness
    fn to_map(&self, default: &DefaultAttr) -> HashMap<String, AttributeValue> {
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
                    index.partition_key.join_composite(&attrs, default),
                )),
            );
            // sort key
            m.insert(
                index.sort_key.field.clone(),
                AttributeValue::S(format!(
                    "${}{}",
                    info.entity,
                    index.sort_key.join_composite(&attrs, default)
                )),
            );
        }
        m
    }

    fn put(&self, c: &Client) -> PutItemFluentBuilder {
        let mut req = c.put_item().table_name(self.info().table);
        let m = self.to_map(&DefaultAttr::Use);
        for (k, v) in &m {
            req = req.item(k, v.clone());
        }
        req
    }

    fn query(&self, client: &Client, index: &str) -> QueryFluentBuilder {
        let is = self.index_schema();
        let i = is.get(index).unwrap();
        let pkf = i.partition_key.field.clone();
        let skf = i.sort_key.field.clone();
        // todo: verify the index composites exist in av
        let av = self.to_map(&DefaultAttr::Ignore);

        client
            .query()
            .table_name(self.info().table)
            .key_condition_expression(format!("#{pkf} = :{pkf} and begins_with(#{skf}, :{skf})"))
            .expression_attribute_names(format!("#{pkf}"), &pkf)
            .expression_attribute_names(format!("#{skf}"), &skf)
            .expression_attribute_values(format!(":{pkf}"), av.get(&pkf).unwrap().clone())
            .expression_attribute_values(format!(":{skf}"), av.get(&skf).unwrap().clone())
    }

    fn from_map(m: &HashMap<String, AttributeValue>) -> Self;

    fn from_map_slice(ms: &[HashMap<String, AttributeValue>]) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut v = Vec::new();
        for a in ms {
            v.push(Self::from_map(a))
        }
        v
    }
}
