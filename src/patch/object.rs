use super::{format, Error};
use crate::{object, schema};
use liferay_object::models::{ObjectDefinition, ObjectField};
use oas::{OpenAPIV3, Operation, ParameterIn, Reference, Referenceable, Schema};
use std::collections::BTreeMap;

/// There are some default settings of a generated object schema that will cause
/// many type generators to fail. This removes those.
pub fn clean_object_schema(
    spec: &mut OpenAPIV3,
    object_def: &ObjectDefinition,
) -> Result<(), Error> {
    let object_name =
        object::extract::object_name(object_def).ok_or(Error::MissingField("objectName"))?;

    let object_schema = schema::extract::object_schema_mut(object_name, spec)
        .ok_or(Error::MissingSchema(object_name.to_owned()))?;

    if let Some(properties) = object_schema.properties.as_mut() {
        properties.values_mut().for_each(|schema| {
            clean_array_type(schema);
        });

        redirect_attachment_fields(object_def, properties);
        redirect_creator(properties);
        // TODO: Detect if attachments are necessary during the load step and patch with attachment type if needed
    };

    object_schema.required = None;

    Ok(())
}

pub fn redirect_creator(properties: &mut BTreeMap<String, Schema>) {
    if let Some(creator) = properties.get_mut("creator") {
        creator._type = None;
        creator._ref = Some("#/components/schemas/Creator".into())
    }
}

/// Remove schema type 'object' for arrays
fn clean_array_type(schema: &mut Schema) {
    if schema
        ._type
        .as_ref()
        .map(|_type| *_type == oas::Type::Array)
        .unwrap_or_default()
    {
        if let Some(items) = schema.items.as_mut() {
            if items
                ._type
                .as_ref()
                .map(|_type| *_type == oas::Type::Object)
                .unwrap_or_default()
            {
                schema._type = None;
            }
        };
    };
}

/// Liferay marks attachments simply as "object". This instead gives a ref to
/// the Attachment schema, which will be patched in by another function if it
/// doesn't yet exist.
fn redirect_attachment_fields(
    object_def: &ObjectDefinition,
    properties: &mut BTreeMap<String, Schema>,
) {
    object::extract::attachment_fields(object_def)
        .iter()
        .for_each(|&field| {
            if let Some(name) = field.name.as_deref() {
                if let Some(schema) = properties.get_mut(name) {
                    schema.one_of = Some(vec![
                        Referenceable::Reference(Reference {
                            _ref: "#/components/schemas/Attachment".into(),
                        }),
                        Referenceable::Data(Schema {
                            _type: Some(oas::Type::Integer),
                            ..Default::default()
                        }),
                    ]);
                    schema._type = None;
                }
            }
        })
}

/// Create enums for field names and ERCs for use in API parameters
pub fn gen_field_enums(spec: &mut OpenAPIV3, object_def: &ObjectDefinition) -> Result<(), Error> {
    let object_name =
        object::extract::object_name(object_def).ok_or(Error::MissingField("name"))?;

    if let Some(schemas) = spec
        .components
        .as_mut()
        .and_then(|comp| comp.schemas.as_mut())
    {
        let std_fields = object::extract::standard_fields(object_def);

        // Create and insert enums from field names
        let field_names = std_fields
            .iter()
            .flat_map(|field| field.name.to_owned())
            .collect::<Vec<_>>();

        let field_name_enum = Schema {
            _enum: Some(field_names),
            _type: Some(oas::Type::String),
            ..Default::default()
        };

        let field_enum_name = format::fieldname_enum_name(object_name);
        schemas.insert(field_enum_name, Referenceable::Data(field_name_enum));

        // Create and insert enums for field ERCs
        let field_ercs = std_fields
            .iter()
            .flat_map(|field| field.external_reference_code.to_owned())
            .collect::<Vec<_>>();

        let field_name_enum = Schema {
            _enum: Some(field_ercs),
            _type: Some(oas::Type::String),
            ..Default::default()
        };
        let erc_enum_name = format::erc_enum_name(object_name);
        schemas.insert(erc_enum_name, Referenceable::Data(field_name_enum));

        // Create and insert enums for relationship fields
        let relationship_fields = object::extract::relationship_fields(object_def);
        let (mut parent_names, all) = extract_relationship_field_names(relationship_fields);
        let relation_name_enum = Schema {
            _enum: Some(all),
            _type: Some(oas::Type::String),
            ..Default::default()
        };
        let relation_enum_name = format::relation_enum_name(object_name);
        schemas.insert(relation_enum_name, Referenceable::Data(relation_name_enum));

        // Create enum for nested field names
        let mut nested_field_names = object_def
            .object_relationships
            .as_ref()
            .map(|rels| {
                rels.iter()
                    .flat_map(|rel| rel.name.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        nested_field_names.append(&mut parent_names);
        let nested_field_enum = Schema {
            _enum: Some(nested_field_names),
            _type: Some(oas::Type::String),
            ..Default::default()
        };
        let relation_enum_name = format::nested_field_enum_name(object_name);
        schemas.insert(relation_enum_name, Referenceable::Data(nested_field_enum));
    }
    Ok(())
}

/// Get all of the relationship field names and parent field names
fn extract_relationship_field_names(
    relationship_fields: Vec<&ObjectField>,
) -> (Vec<String>, Vec<String>) {
    relationship_fields
        .iter()
        .fold((vec![], vec![]), |(mut parents, mut all), field| {
            if let Some(name) = field.name.as_deref() {
                all.push(name.to_owned());
                if let Some(parent_name) = name.split('_').nth(1) {
                    parents.push(parent_name.to_owned())
                }
            }
            if let Some(settings) = field.object_field_settings.as_ref() {
                settings
                    .iter()
                    .filter(|setting| {
                        setting
                            .name
                            .as_deref()
                            .map(|name| name == "objectRelationshipERCObjectFieldName")
                            .unwrap_or_default()
                    })
                    .flat_map(|setting| setting.value.as_ref())
                    .map(|value| value.to_string())
                    .for_each(|value| all.push(value));
            };

            (parents, all)
        })
}

pub fn gen_field_enum_references(
    object_def: &ObjectDefinition,
    spec: &mut OpenAPIV3,
) -> Result<(), Error> {
    println!("CALLED");
    let is_system_object = object_def.system.unwrap_or_default();

    if !is_system_object {
        println!("CALLED");
        let object_name =
            object::extract::object_name(object_def).ok_or(Error::MissingField("name"))?;
        let relation_field_count = object::extract::relationship_fields(object_def).len();

        spec.paths
            .values_mut()
            .filter_map(|path| path.get.as_mut())
            .filter(|get| {
                get.operation_id
                    .as_deref()
                    .map(|op_id| op_id != "getOpenAPI")
                    .unwrap_or_default()
            })
            .filter_map(|get: &mut Operation| get.parameters.as_mut())
            .for_each(|params| {
                dbg!(&params);
                let system_field_name_ref = Schema {
                    _ref: Some("#/components/schemas/SystemFieldName".into()),
                    ..Default::default()
                };
                let field_name_ref = Schema {
                    _ref: Some(format!("#/components/schemas/{object_name}FieldName")),
                    ..Default::default()
                };
                let nested_field_name_ref = Schema {
                    _ref: Some(format!("#/components/schemas/{object_name}NestedFieldName")),
                    ..Default::default()
                };
                let relation_field_name_ref = Schema {
                    _ref: Some(format!(
                        "#/components/schemas/{object_name}RelationshipFieldName"
                    )),
                    ..Default::default()
                };
                let string_type = Schema {
                    _type: Some(oas::Type::String),
                    ..Default::default()
                };

                let items = {
                    let mut items = vec![system_field_name_ref, field_name_ref, string_type];
                    if relation_field_count > 0 {
                        items.push(relation_field_name_ref);
                        items.push(nested_field_name_ref); // TODO: check if error and fix
                    };
                    items
                };
                params.iter_mut().for_each(|param| {
                    if let Referenceable::Data(param) = param {
                        if param.name == "restrictFields"
                            || param.name == "fields"
                            || param.name == "nestedFields"
                        {
                            param._in = ParameterIn::Query;
                            param.style = Some(oas::Style::Form);
                            param.explode = Some(true);
                        }
                        if param.name == "restrictFields" || param.name == "fields" {
                            param.schema = Some(Schema {
                                _type: Some(oas::Type::Array),
                                items: Some(Box::new(Schema {
                                    one_of: Some(
                                        items
                                            .iter()
                                            .map(|item| Referenceable::Data(item.to_owned()))
                                            .collect::<Vec<_>>(),
                                    ),
                                    ..Default::default()
                                })),
                                ..Default::default()
                            })
                        } else if param.name == "nestedFields" {
                            // TODO: handle case where there are no nested fields
                            param.schema = Some(Schema {
                                _type: Some(oas::Type::Array),
                                items: Some(Box::new(Schema {
                                    _ref: Some(format!(
                                        "#/components/schemas/{object_name}NestedFieldName"
                                    )),
                                    ..Default::default()
                                })),
                                ..Default::default()
                            })
                        }
                    };
                });
            });
    };
    Ok(())
}
