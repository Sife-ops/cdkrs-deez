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
pub struct IndexSchema {
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

    fn index_schemas(&self) -> HashMap<Index, IndexSchema>;

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
        let is = self.index_schemas();
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

    pub fn put(&self, entity: &impl DeezEntity) -> PutItemFluentBuilder {
        let mut req = self.client.put_item().table_name(entity.info().table);
        let m = entity.to_map();
        for (k, v) in &m {
            req = req.item(k, v.clone());
        }
        req
    }

    pub fn query(&self, index: Index, entity: &impl DeezEntity) -> QueryFluentBuilder {
        let is = entity.index_schemas();
        let i = is.get(&index).unwrap();
        let pkf = i.partition_key.field.clone();
        let skf = i.sort_key.field.clone();
        // todo: verify the index composites exist in av
        let av = entity.to_map();

        let mut request = self
            .client
            .query()
            .table_name(entity.info().table)
            .key_condition_expression(format!("#{pkf} = :{pkf} and begins_with(#{skf}, :{skf})"))
            .expression_attribute_names(format!("#{pkf}"), &pkf)
            .expression_attribute_names(format!("#{skf}"), &skf)
            .expression_attribute_values(format!(":{pkf}"), av.get(&pkf).unwrap().clone())
            .expression_attribute_values(format!(":{skf}"), av.get(&skf).unwrap().clone());

        if index != Index::Primary {
            request = request.index_name(index.to_string());
        }

        request
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum Index<'a> {
    Primary,
    Gsi1(&'a str),
    Gsi2(&'a str),
    Gsi3(&'a str),
    Gsi4(&'a str),
    Gsi5(&'a str),
    Gsi6(&'a str),
    Gsi7(&'a str),
    Gsi8(&'a str),
    Gsi9(&'a str),
    Gsi10(&'a str),
    Gsi11(&'a str),
    Gsi12(&'a str),
    Gsi13(&'a str),
    Gsi14(&'a str),
    Gsi15(&'a str),
    Gsi16(&'a str),
    Gsi17(&'a str),
    Gsi18(&'a str),
    Gsi19(&'a str),
    Gsi20(&'a str),
}

impl std::fmt::Display for Index<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Index::Primary => write!(f, "primary"),
            Index::Gsi1(x) => write!(f, "{}", x),
            Index::Gsi2(x) => write!(f, "{}", x),
            Index::Gsi3(x) => write!(f, "{}", x),
            Index::Gsi4(x) => write!(f, "{}", x),
            Index::Gsi5(x) => write!(f, "{}", x),
            Index::Gsi6(x) => write!(f, "{}", x),
            Index::Gsi7(x) => write!(f, "{}", x),
            Index::Gsi8(x) => write!(f, "{}", x),
            Index::Gsi9(x) => write!(f, "{}", x),
            Index::Gsi10(x) => write!(f, "{}", x),
            Index::Gsi11(x) => write!(f, "{}", x),
            Index::Gsi12(x) => write!(f, "{}", x),
            Index::Gsi13(x) => write!(f, "{}", x),
            Index::Gsi14(x) => write!(f, "{}", x),
            Index::Gsi15(x) => write!(f, "{}", x),
            Index::Gsi16(x) => write!(f, "{}", x),
            Index::Gsi17(x) => write!(f, "{}", x),
            Index::Gsi18(x) => write!(f, "{}", x),
            Index::Gsi19(x) => write!(f, "{}", x),
            Index::Gsi20(x) => write!(f, "{}", x),
        }
    }
}
