use std::collections::BTreeMap;

use liferay_object::models::{ObjectDefinition, ObjectField};
use openapi::v3_0::{ObjectOrReference, Schema, Spec};

use crate::{object, schema};

use super::{format, Error};

/// There are some default settings of a generated object schema that will cause
/// many type generators to fail. This removes those.
pub fn clean_object_schema(spec: &mut Spec, object_def: &ObjectDefinition) -> Result<(), Error> {
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
        creator.schema_type = None;
        creator.ref_path = Some("#/components/schemas/Creator".into())
    }
}

/// Remove schema type 'object' for arrays
fn clean_array_type(schema: &mut Schema) {
    if schema
        .schema_type
        .as_ref()
        .map(|schema_type| schema_type == "array")
        .unwrap_or_default()
    {
        if let Some(items) = schema.items.as_mut() {
            if items
                .schema_type
                .as_ref()
                .map(|schema_type| schema_type == "object")
                .unwrap_or_default()
            {
                schema.schema_type = None;
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
                        ObjectOrReference::Ref {
                            ref_path: "#/components/schemas/Attachment".into(),
                        },
                        ObjectOrReference::Object(Schema {
                            schema_type: Some("integer".into()),
                            ..Default::default()
                        }),
                    ]);
                    schema.schema_type = None;
                }
            }
        })
}

/// Create enums for field names and ERCs for use in API parameters
pub fn gen_field_enums(spec: &mut Spec, object_def: &ObjectDefinition) -> Result<(), Error> {
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
            enum_values: Some(field_names),
            schema_type: Some("string".into()),
            ..Default::default()
        };

        let field_enum_name = format::fieldname_enum_name(object_name);
        schemas.insert(field_enum_name, ObjectOrReference::Object(field_name_enum));

        // Create and insert enums for field ERCs
        let field_ercs = std_fields
            .iter()
            .flat_map(|field| field.external_reference_code.to_owned())
            .collect::<Vec<_>>();

        let field_name_enum = Schema {
            enum_values: Some(field_ercs),
            schema_type: Some("string".into()),
            ..Default::default()
        };
        let erc_enum_name = format::erc_enum_name(object_name);
        schemas.insert(erc_enum_name, ObjectOrReference::Object(field_name_enum));

        // Create and insert enums for relationship fields
        let relationship_fields = object::extract::relationship_fields(object_def);
        let (mut parent_names, all) = extract_relationship_field_names(relationship_fields);
        let relation_name_enum = Schema {
            enum_values: Some(all),
            schema_type: Some("string".into()),
            ..Default::default()
        };
        let relation_enum_name = format::relation_enum_name(object_name);
        schemas.insert(
            relation_enum_name,
            ObjectOrReference::Object(relation_name_enum),
        );

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
            enum_values: Some(nested_field_names),
            schema_type: Some("string".into()),
            ..Default::default()
        };
        let relation_enum_name = format::nested_field_enum_name(object_name);
        schemas.insert(
            relation_enum_name,
            ObjectOrReference::Object(nested_field_enum),
        );
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
    spec: &mut Spec,
) -> Result<(), Error> {
    let is_system_object = object_def.system.unwrap_or_default();
    if !is_system_object {
        let paths = spec
            .paths
            .values_mut()
            .filter_map(|path| path.get.as_mut())
            .filter(|get| {
                get.operation_id
                    .as_deref()
                    .map(|op_id| op_id != "getOpenAPI")
                    .unwrap_or_default()
            })
            .filter_map(|get: &mut openapi::v3_0::Operation| get.parameters.as_mut())
            .map(|params| {
                let params = params.iter_mut().map(|param| {
                    if let ObjectOrReference::Object(ref_obj) = param {};
                    todo!();
                });
            });
        // TODO: finish on line 480-ish
    };
    Ok(())
}
