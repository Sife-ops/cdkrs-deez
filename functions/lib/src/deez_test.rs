#[cfg(test)]
mod dynamo_test {
    use crate::deez::{Attribute, DeezEntity, EntityInfo, Index, Key};
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

    impl DeezEntity for TestStruct {
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
                format!("primaryid") => Attribute::DdbString(self.primary_id.clone()),
                format!("foreignid") => Attribute::DdbString(self.foreign_id.clone()),
                format!("createdat") => Attribute::DdbString(self.created_at.clone()),
            }
        }

        fn generated_values() -> Self {
            TestStruct {
                primary_id: Some(Uuid::new_v4().to_string()),
                created_at: Some(Utc::now().to_rfc3339()),
                ..Default::default()
            }
        }

        fn from_map(m: &HashMap<String, AttributeValue>) -> Self {
            let mut d = TestStruct {
                ..Default::default()
            };
            for (k, v) in m {
                match k.as_str() {
                    "primaryid" => d.primary_id = Some(v.as_s().unwrap().clone()), // todo: clone or to_string
                    "foreignid" => d.foreign_id = Some(v.as_s().unwrap().clone()),
                    "createdat" => d.created_at = Some(v.as_s().unwrap().clone()),
                    &_ => {}
                }
            }
            d
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
            ..TestStruct::generated_values()
        };
        println!("");
        println!("{:?}", a);
        let avm = a.to_map();
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
        let avm = a.to_map();

        sugon(&avm);

        assert_eq!(deez(&avm, "pk"), format!("$TestService#TestEntity"));
        assert_eq!(deez(&avm, "sk"), format!("$TestEntity"));
        assert_eq!(deez(&avm, "gsi1pk"), format!("$TestService#TestEntity"));
        assert_eq!(deez(&avm, "gsi1sk"), format!("$TestEntity"));
        assert_eq!(deez(&avm, "gsi2pk"), format!("$TestService#TestEntity"));
        assert_eq!(deez(&avm, "gsi2sk"), format!("$TestEntity"));
    }

    // #[test]
    // fn e2m3() {
    //     let a = TestStruct {
    //         primary_id: Some(format!("AAA")),
    //         ..Default::default()
    //     };
    //     let avm = a.to_map();
    //     sugon(&avm);
    // }
}
