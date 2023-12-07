use crate::{config::Config, load::MetaData};
use openapi::v3_0::Spec;

use self::{
    object::{clean_object_schema, gen_field_enum_references, gen_field_enums},
    picklist::{gen_picklist_enum_references, gen_picklist_enums},
};

mod error;
mod format;
mod picklist;
mod relation;
pub use error::Error;
mod object;

pub fn patch(config: &Config, metadata: MetaData) -> Result<Spec, Error> {
    let mut spec = metadata.spec;

    if let Some(ref object_def) = metadata.object_def {
        // remove_relations(object_def, &mut spec); // I can't remember why we do this, so for now we comment it out and may return it later

        // Carry out processing specific to objects that include picklists.
        if let Some(picklists) = metadata.picklists {
            gen_picklist_enums(config, picklists, &mut spec);
            gen_picklist_enum_references(object_def, &mut spec)?;
        }

        clean_object_schema(&mut spec, object_def)?;
        gen_field_enums(&mut spec, object_def)?;
        gen_field_enum_references(object_def, &mut spec)?;
    }

    Ok(spec)
}

#[cfg(test)]
mod test {
    use convert_case::{Case, Casing};
    use lazy_static::lazy_static;
    use liferay_object::models::ObjectDefinition;
    use list_type::models::ListTypeDefinition;
    use openapi::v3_0::{ObjectOrReference, Spec};

    use crate::{config::Config, load::MetaData, patch::patch};

    struct TestResult {
        pub spec: Spec,
        pub metadata: MetaData,
    }

    impl TestResult {
        fn get_picklist_names(&self) -> Vec<&str> {
            self.metadata
                .picklists
                .as_ref()
                .map(|picklists| {
                    picklists
                        .iter()
                        .flat_map(|picklist| picklist.external_reference_code.as_deref())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        }

        fn get_picklist_item_ercs_by_picklist_erc(&self, erc: &str) -> Vec<&str> {
            self.metadata
                .picklists
                .as_ref()
                .and_then(|picklists| {
                    picklists
                        .iter()
                        .find(|picklist| {
                            picklist
                                .external_reference_code
                                .as_ref()
                                .and_then(|_erc| if erc == _erc { Some(picklist) } else { None })
                                .is_some()
                        })
                        .and_then(|picklist| {
                            picklist.list_type_entries.as_ref().map(|entries| {
                                entries
                                    .iter()
                                    .flat_map(|entry| entry.external_reference_code.as_deref())
                                    .collect::<Vec<_>>()
                            })
                        })
                })
                .unwrap_or_default()
        }
    }

    lazy_static! {
        static ref CUSTOM_OBJECT_RESULT: TestResult = {
            let object_def = std::fs::read_to_string("./src/patch/test_files/test_def.json")
                .expect("Failed to find test object def");
            let object_def: ObjectDefinition =
                serde_json::from_str(&object_def).expect("Failed to deserialize object definition");
            let spec = std::fs::read_to_string("./src/patch/test_files/test_spec.json")
                .expect("Failed to find test open api spec");
            let spec: Spec =
                serde_json::from_str(&spec).expect("Failed to deserialize open api spec");
            let picklists = std::fs::read_to_string("./src/patch/test_files/test_picklists.json")
                .expect("Failed to find test picklists file");
            let picklists: Vec<ListTypeDefinition> =
                serde_json::from_str(&picklists).expect("Failed to list type definitions");
            let metadata = MetaData {
                spec,
                object_def: Some(object_def),
                picklists: Some(picklists),
            };

            let spec = patch(&Config::default(), metadata.clone())
                .expect("Should have not errored when patching the spec");

            TestResult { spec, metadata }
        };
    }

    #[test]
    fn creator_should_be_ref_to_creator() {
        CUSTOM_OBJECT_RESULT.spec.components.as_ref().map(|comp| {
            comp.schemas.as_ref().map(|schemas| {
                schemas.values().for_each(|schema| {
                    if let ObjectOrReference::Object(schema_obj) = schema {
                        schema_obj.properties.as_ref().map(|properties| {
                            properties.get("creator").map(|creator| {
                                assert_eq!(creator.schema_type, None);
                                assert_eq!(
                                    creator.ref_path,
                                    Some("#/components/schemas/Creator".into())
                                )
                            })
                        });
                    };
                })
            })
        });
    }

    #[test]
    fn picklists_should_be_available_as_erc_enum() {
        let picklist_names = CUSTOM_OBJECT_RESULT
            .get_picklist_names()
            .iter()
            .map(|name| name.to_case(Case::Camel))
            .collect::<Vec<_>>();

        CUSTOM_OBJECT_RESULT
            .spec
            .components
            .as_ref()
            .map(|components| {
                components.schemas.as_ref().map(|schemas| {
                    picklist_names.iter().for_each(|erc| {
                        assert!(schemas.get(erc).is_some());
                    })
                })
            });
    }

    #[test]
    fn picklists_enum_values_should_be_same_as_picklist_item_ercs() {
        let picklist_names = CUSTOM_OBJECT_RESULT.get_picklist_names();

        CUSTOM_OBJECT_RESULT
            .spec
            .components
            .as_ref()
            .map(|components| {
                components.schemas.as_ref().map(|schemas| {
                    picklist_names.iter().for_each(|erc| {
                        let schema = schemas
                            .get(&erc.to_case(Case::Camel))
                            .expect("There should be a schema for a given erc");
                        let schema = match schema {
                            ObjectOrReference::Object(schema) => schema,
                            ObjectOrReference::Ref { .. } => {
                                panic!("Picklist shold not be a ref string")
                            }
                        };
                        let picklist_items =
                            CUSTOM_OBJECT_RESULT.get_picklist_item_ercs_by_picklist_erc(erc);
                        let mut values = schema
                            .enum_values
                            .as_ref()
                            .expect("there should be enum values")
                            .clone();

                        assert_eq!(picklist_items.len(), values.len());
                        values.sort();
                        values
                            .iter()
                            .zip(picklist_items)
                            .for_each(|(a, b)| assert_eq!(a, b))
                    })
                })
            });
    }

    #[test]
    fn picklists_should_be_available_as_key_enum() {
        let picklist_names = CUSTOM_OBJECT_RESULT
            .get_picklist_names()
            .iter()
            .map(|name| name.to_case(Case::Camel))
            .collect::<Vec<_>>();

        CUSTOM_OBJECT_RESULT
            .spec
            .components
            .as_ref()
            .map(|components| {
                components.schemas.as_ref().map(|schemas| {
                    picklist_names.iter().for_each(|erc| {
                        assert!(schemas.get(&format!("{erc}Key")).is_some());
                    })
                })
            });
    }

    #[test]
    fn picklists_should_be_available_as_item_object() {
        let picklist_names = CUSTOM_OBJECT_RESULT
            .get_picklist_names()
            .iter()
            .map(|name| name.to_case(Case::Camel))
            .collect::<Vec<_>>();

        CUSTOM_OBJECT_RESULT
            .spec
            .components
            .as_ref()
            .map(|components| {
                components.schemas.as_ref().map(|schemas| {
                    picklist_names.iter().for_each(|erc| {
                        assert!(schemas.get(&format!("{erc}Item")).is_some());
                    })
                })
            });
    }

    #[test]
    fn creates_enum_for_field_names() {
        let object_name = CUSTOM_OBJECT_RESULT
            .metadata
            .object_def
            .as_ref()
            .and_then(|def| def.name.as_deref())
            .expect("Should have an object name");

        let field_enum_name = format!("{object_name}FieldName");
        let field_enum = CUSTOM_OBJECT_RESULT
            .spec
            .components
            .as_ref()
            .and_then(|comp| {
                comp.schemas
                    .as_ref()
                    .and_then(|schemas| schemas.get(&field_enum_name))
            })
            .expect("the field enum should exist");

        match field_enum {
            ObjectOrReference::Ref { .. } => panic!("Field enum should not be ref"),
            ObjectOrReference::Object(..) => (),
        };
    }
}
