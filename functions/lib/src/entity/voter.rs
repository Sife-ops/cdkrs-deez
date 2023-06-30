use deez::*;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Deez)]
#[deez_schema(table = "prod-glsst-table", service = "Glsst", entity = "User")]
#[deez_schema(primary_hash = "pk", primary_range = "sk")]
#[deez_schema(gsi1_name = "gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
#[deez_schema(gsi2_name = "gsi2", gsi2_hash = "gsi2pk", gsi2_range = "gsi2sk")]
pub struct Voter {
    #[deez_primary(key = "hash")]
    pub voterid: Option<String>,
    #[deez_gsi1(key = "range")]
    #[deez_gsi2(key = "hash")]
    pub predictionid: Option<String>,
    #[deez_gsi1(key = "hash")]
    #[deez_gsi2(key = "range")]
    pub userid: Option<String>,
    pub vote: bool,
}

impl Default for Voter {
    fn default() -> Self {
        Voter {
            voterid: Some(Uuid::new_v4().to_string()),
            predictionid: None,
            userid: None,
            vote: false,
        }
    }
}

//     fn generated_values() -> Self {
//         Voter {
//             voter_id: Uuid::new_v4().to_string(),
//             ..Default::default()
//         }
//     }

// // todo: https://users.rust-lang.org/t/access-struct-attributes-by-string/17520/2
// impl DeezEntity for Voter {
//     fn info(&self) -> EntityInfo {
//         EntityInfo {
//             table: env::var("TABLE_NAME").unwrap_or(format!("MISSING")),
//             service: format!("Glsst"),
//             entity: format!("Voter"),
//         }
//     }

//     fn index_schemas(&self) -> HashMap<Index, IndexSchema> {
//         hashmap! {
//             indexes::PRIMARY => {
//                 IndexSchema {
//                     partition_key: Key {
//                         field: format!("pk"),
//                         composite: vec![format!("voterid")],
//                     },
//                     sort_key: Key {
//                         field: format!("sk"),
//                         composite: vec![],
//                     },
//                 }
//             },
//             indexes::GSI1 => {
//                 IndexSchema {
//                     partition_key: Key {
//                         field: format!("gsi1pk"),
//                         composite: vec![format!("userid")],
//                     },
//                     sort_key: Key {
//                         field: format!("gsi1sk"),
//                         composite: vec![format!("predictionid")],
//                     },
//                 }
//             },
//             indexes::GSI2 => {
//                 IndexSchema {
//                     partition_key: Key {
//                         field: format!("gsi2pk"),
//                         composite: vec![format!("predictionid")],
//                     },
//                     sort_key: Key {
//                         field: format!("gsi2sk"),
//                         composite: vec![format!("userid")],
//                     },
//                 }
//             },
//         }
//     }

//     fn attributes(&self) -> HashMap<String, Attribute> {
//         hashmap! {
//             format!("voterid") => Attribute::DeezString(self.voter_id.clone()),
//             format!("predictionid") => Attribute::DeezString(self.prediction_id.clone()),
//             format!("userid") => Attribute::DeezString(self.user_id.clone()),
//             format!("vote") => Attribute::DeezBoolean(self.vote.clone()),
//         }
//     }

//     fn from_map(m: &HashMap<String, AttributeValue>) -> Self {
//         let mut x = Voter {
//             ..Default::default()
//         };
//         for (k, v) in m {
//             match k.as_str() {
//                 "voterid" => x.voter_id = v.as_s().unwrap().clone(),
//                 "predictionid" => x.prediction_id = v.as_s().unwrap().clone(),
//                 "userid" => x.user_id = v.as_s().unwrap().clone(),
//                 "vote" => x.vote = v.as_bool().unwrap().clone(),
//                 &_ => {}
//             }
//         }
//         x
//     }
// }
