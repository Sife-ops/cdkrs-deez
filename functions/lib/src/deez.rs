// todo: rename to deez.rs
use aws_sdk_dynamodb::operation::put_item::builders::PutItemFluentBuilder;
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

pub struct EntityInfo {
    pub table: String,
    pub service: String,
    pub entity: String,
}

#[derive(Debug)]
pub struct Index {
    pub partition_key: Key,
    pub sort_key: Key,
}

#[derive(Debug)]
pub struct Key {
    pub field: String,
    pub composite: Vec<String>,
}

impl Key {
    fn join_composite(&self, attrs: &HashMap<String, Attribute>) -> String {
        let mut c = String::new();
        for composite in self.composite.iter() {
            let a = attrs.get(composite).unwrap();
            let s = a.to_string();
            if s.len() > 0 {
                c.push_str(&format!("#{}_{}", composite, s,));
            }
        }
        c
    }
}

#[derive(Debug, Clone)]
pub enum Attribute {
    DeezString(String),
    DeezNumber(isize),
    DeezBoolean(bool),
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::DeezString(x) => {
                write!(f, "{}", x)
            }
            Attribute::DeezNumber(x) => {
                write!(f, "{}", x)
            }
            Attribute::DeezBoolean(x) => {
                write!(f, "{}", x)
            }
        }
    }
}

#[derive(PartialEq)]
pub enum GeneratedValues {
    Use,
    Ignore,
}

pub trait DeezEntity {
    fn info(&self) -> EntityInfo;

    fn index_schema(&self) -> HashMap<String, Index>;

    fn attributes(&self) -> HashMap<String, Attribute>;

    fn generated_values() -> Self;

    fn to_map(&self) -> HashMap<String, AttributeValue> {
        let mut m = HashMap::new();
        m.insert(
            format!("_entity"),
            AttributeValue::S(self.info().entity.clone()),
        );

        // attributes
        let attrs = &self.attributes();
        for (k, attr) in attrs {
            match attr {
                Attribute::DeezString(x) => {
                    m.insert(k.to_string(), AttributeValue::S(x.to_string()));
                }
                Attribute::DeezBoolean(x) => {
                    m.insert(k.to_string(), AttributeValue::Bool(*x));
                }
                Attribute::DeezNumber(x) => {
                    m.insert(k.to_string(), AttributeValue::N(x.to_string()));
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
                    self.info().service,
                    self.info().entity,
                    index.partition_key.join_composite(attrs),
                )),
            );
            // sort key
            m.insert(
                index.sort_key.field.clone(),
                AttributeValue::S(format!(
                    "${}{}",
                    self.info().entity,
                    index.sort_key.join_composite(attrs)
                )),
            );
        }

        m
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

pub struct Deez {
    client: Client,
}

impl Deez {
    pub fn new(c: Client) -> Self {
        Self { client: c }
    }

    pub fn put(&self, e: &impl DeezEntity) -> PutItemFluentBuilder {
        let mut req = self.client.put_item().table_name(e.info().table);
        let m = e.to_map();
        for (k, v) in &m {
            req = req.item(k, v.clone());
        }
        req
    }

    pub fn query(&self, index: &str, e: &impl DeezEntity) -> QueryFluentBuilder {
        let is = e.index_schema();
        let i = is.get(index).unwrap();
        let pkf = i.partition_key.field.clone();
        let skf = i.sort_key.field.clone();
        // todo: verify the index composites exist in av
        let av = e.to_map();

        let mut request = self
            .client
            .query()
            .table_name(e.info().table)
            .key_condition_expression(format!("#{pkf} = :{pkf} and begins_with(#{skf}, :{skf})"))
            .expression_attribute_names(format!("#{pkf}"), &pkf)
            .expression_attribute_names(format!("#{skf}"), &skf)
            .expression_attribute_values(format!(":{pkf}"), av.get(&pkf).unwrap().clone())
            .expression_attribute_values(format!(":{skf}"), av.get(&skf).unwrap().clone());

        // todo: WARANING! EXTREMELY SUS!
        // hav 2 make index and enum bro
        if index != "primary" {
            request = request.index_name(index);
        }

        request
    }
}
