#[cfg(test)]
mod dynamo_test {
    use crate::dynamo::{Attribute, DdbEntity, DefaultAttr, EntityInfo, Index, Key, Value};
    use aws_sdk_dynamodb::types::AttributeValue;
    use chrono::Utc;
    use maplit::hashmap;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[derive(Default, Debug)]
    struct TestStruct {
        primary_id: Option<String>,
        foreign_id: Option<String>,
        created_at: Option<String>,
    }

    impl DdbEntity for TestStruct {
        fn info(&self) -> EntityInfo {
            EntityInfo {
                table: format!("test_table"),
                service: format!("TestService"),
                entity: format!("TestEntity"),
            }
        }

        fn index_schema(&self) -> HashMap<String, Index> {
            hashmap! {
                format!("primary") => {
                    Index {
                        partition_key: Key {
                            field: format!("pk"),
                            composite: vec![format!("primaryid")],
                        },
                        sort_key: Key {
                            field: format!("sk"),
                            composite: vec![],
                        },
                    }
                },
                format!("gsi1") => {
                    Index {
                        partition_key: Key {
                            field: format!("gsi1pk"),
                            composite: vec![format!("foreignid")],
                        },
                        sort_key: Key {
                            field: format!("gsi1sk"),
                            composite: vec![format!("primaryid")],
                        },
                    }
                },
                format!("gsi2") => {
                    Index {
                        partition_key: Key {
                            field: format!("gsi2pk"),
                            composite: vec![format!("primaryid"), format!("foreignid")],
                        },
                        sort_key: Key {
                            field: format!("gsi2sk"),
                            composite: vec![format!("createdat")],
                        },
                    }
                },
            }
        }

        fn attributes(&self) -> HashMap<String, Attribute> {
            hashmap! {
                format!("primaryid") => Attribute::DdbString(Value {
                    value: self.primary_id.clone(),
                    default: Some(Uuid::new_v4().to_string()),
                }),
                format!("foreignid") => Attribute::DdbString(Value {
                    value: self.foreign_id.clone(),
                    default: None,
                }),
                format!("createdat") => Attribute::DdbString(Value {
                    value: self.created_at.clone(),
                    default: Some(Utc::now().to_rfc3339()),
                }),
            }
        }
    }

    fn deez(a: &HashMap<String, AttributeValue>, b: &str) -> String {
        a.get(b).unwrap().as_s().unwrap().to_string()
    }

    fn sugon(a: &HashMap<String, AttributeValue>) {
        println!("");
        for (k, v) in a {
            println!("{}: {:?}", k, v);
        }
    }

    #[test]
    fn e2m1() {
        let fid = Uuid::new_v4().to_string();
        let a = TestStruct {
            foreign_id: Some(fid.clone()),
            ..Default::default()
        };
        let avm = a.entity_to_av_map(&DefaultAttr::Use);
        let pid = Uuid::parse_str(&deez(&avm, "primaryid"))
            .unwrap()
            .to_string();
        let ca = deez(&avm, "createdat");

        sugon(&avm);

        assert_eq!(
            deez(&avm, "pk"),
            format!("$TestService#TestEntity#primaryid_{pid}")
        );
        assert_eq!(deez(&avm, "sk"), format!("$TestEntity"));

        assert_eq!(
            deez(&avm, "gsi1pk"),
            format!("$TestService#TestEntity#foreignid_{fid}")
        );
        assert_eq!(deez(&avm, "gsi1sk"), format!("$TestEntity#primaryid_{pid}"));

        assert_eq!(
            deez(&avm, "gsi2pk"),
            format!("$TestService#TestEntity#primaryid_{pid}#foreignid_{fid}")
        );
        assert_eq!(deez(&avm, "gsi2sk"), format!("$TestEntity#createdat_{ca}"));
    }

    #[test]
    fn e2m2() {
        let a = TestStruct {
            ..Default::default()
        };
        let avm = a.entity_to_av_map(&DefaultAttr::Ignore);

        sugon(&avm);

        assert_eq!(deez(&avm, "pk"), format!("$TestService#TestEntity"));
        assert_eq!(deez(&avm, "sk"), format!("$TestEntity"));
        assert_eq!(deez(&avm, "gsi1pk"), format!("$TestService#TestEntity"));
        assert_eq!(deez(&avm, "gsi1sk"), format!("$TestEntity"));
        assert_eq!(deez(&avm, "gsi2pk"), format!("$TestService#TestEntity"));
        assert_eq!(deez(&avm, "gsi2sk"), format!("$TestEntity"));
    }

    #[test]
    fn e2m3() {
        let a = TestStruct {
            primary_id: Some(format!("AAA")),
            ..Default::default()
        };
        let avm = a.entity_to_av_map(&DefaultAttr::Ignore);
        sugon(&avm);
    }
}
