use crate::{config::Config, error::Reporter, object, schema};

use super::{
    format::{schema_refpath_picklist_key, schema_refpath_piclist_item},
    Error,
};
use convert_case::Casing;
use liferay_object::models::ObjectDefinition;
use list_type::models::ListTypeDefinition;
use openapi::v3_0::{Components, ObjectOrReference, Schema, Spec};
use std::collections::BTreeMap;

/// Open api specification allows defining enums, which are very useful when handling picklist
/// inputs. However, Liferay's open api specification completely ignores this feature and instead
/// models picklists as raw strings. This patch fixes this issue by converting picklist values
/// to openapi enums.
pub fn gen_picklist_enums(config: &Config, picklists: Vec<ListTypeDefinition>, spec: &mut Spec) {
    let (enums, errors): (Vec<_>, Vec<_>) = picklists
        .into_iter()
        .map(picklist_to_enum(config))
        .partition(Result::is_ok);

    let enums: Vec<_> = enums.into_iter().flat_map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

    for error in errors {
        error.report("WARNING: failed to convert picklist to enum");
    }

    let schemas = spec
        .components
        .get_or_insert(Components::default())
        .schemas
        .get_or_insert(BTreeMap::new());

    enums.into_iter().for_each(|(key, enum_schema)| {
        schemas.insert(key, ObjectOrReference::Object(enum_schema));
    });
}

pub fn picklist_to_enum(
    config: &Config,
) -> impl Fn(ListTypeDefinition) -> Result<[(String, Schema); 3], Error> + '_ {
    |picklist: ListTypeDefinition| {
        let erc = picklist
            .external_reference_code
            .as_ref()
            .ok_or(Error::MissingField("externalReferenceCode"))?
            .to_case(config.patch.enum_case.into());

        let erc_enum = build_erc_enum(&picklist, &erc)?;
        let key_enum = build_key_enum(&picklist, &erc)?;
        let item = build_item(&erc);
        Ok([erc_enum, key_enum, item])
    }
}

fn build_erc_enum(picklist: &ListTypeDefinition, erc: &str) -> Result<(String, Schema), Error> {
    let erc_enum_opts: Vec<String> = picklist
        .list_type_entries
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|entry| entry.external_reference_code.clone())
        .collect();

    Ok((
        erc.to_owned(),
        Schema {
            schema_type: Some("string".into()),
            example: Some(serde_json::to_value(
                [erc_enum_opts
                    .get(0)
                    .map(|s| s.to_owned())
                    .unwrap_or_default(); 1],
            )?),
            enum_values: Some(erc_enum_opts),
            ..Default::default()
        },
    ))
}

fn build_key_enum(picklist: &ListTypeDefinition, erc: &str) -> Result<(String, Schema), Error> {
    let key_opts: Vec<String> = picklist
        .list_type_entries
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|entry| entry.key.to_owned())
        .collect();

    Ok((
        format!("{erc}Key"),
        Schema {
            schema_type: Some("string".into()),
            example: Some(serde_json::to_value(
                [key_opts.get(0).map(|s| s.to_owned()).unwrap_or_default(); 1],
            )?),
            enum_values: Some(key_opts),
            description: Some(erc.into()),
            ..Default::default()
        },
    ))
}

fn build_item(erc: &str) -> (String, Schema) {
    let mut properties = BTreeMap::new();
    properties.insert(
        "key".into(),
        Schema {
            ref_path: Some(format!("#/components/schemas/${erc}Key")),
            ..Default::default()
        },
    );
    properties.insert(
        "name".into(),
        Schema {
            schema_type: Some("string".into()),
            ..Default::default()
        },
    );

    (
        format!("{erc}Item"),
        Schema {
            schema_type: Some("object".into()),
            properties: Some(properties),
            required: Some(vec!["key".into()]),
            ..Default::default()
        },
    )
}

/// When we generate picklist enums with [gen_picklist_enums], we need to also attach
/// references to those picklists for the object that uses them. This function is used
/// for that.
pub fn gen_picklist_enum_references(
    object_def: &ObjectDefinition,
    spec: &mut Spec,
) -> Result<(), Error> {
    let object_name =
        object::extract::object_name(object_def).ok_or(Error::MissingField("objectName"))?;
    let object_schema = schema::extract::object_schema_mut(object_name, spec)
        .ok_or(Error::MissingSchema(object_name.to_owned()))?;

    let picklist_fields = object::extract::picklist_fields(object_def);

    for picklist_field in picklist_fields.into_iter() {
        let field_name = picklist_field
            .name
            .as_deref()
            .ok_or(Error::MissingField("name"))?;
        let picklist_schema = object_schema
            .properties
            .as_mut()
            .ok_or(Error::MissingField("properties"))?
            .get_mut(field_name)
            .ok_or(Error::InvalidSchema("Picklist field schema missing"))?;

        let picklist_erc = picklist_field
            .list_type_definition_external_reference_code
            .as_deref()
            .ok_or(Error::InvalidObjectDef(
                "Missing picklist field external reference code",
            ))?;

        let business_type = picklist_field
            .business_type
            .as_ref()
            .ok_or(Error::MissingField("business_type"))?;

        match business_type {
                liferay_object::models::object_field::BusinessType::MultiselectPicklist => {
                    picklist_schema.items.get_or_insert(Box::<Schema>::default()).one_of = gen_picklist_references(picklist_erc);
                    picklist_schema.schema_type = Some("array".into());
                },
                liferay_object::models::object_field::BusinessType::Picklist => {
                    gen_picklist_references(picklist_erc);
                    picklist_schema.schema_type = None;
                },
                _ => Err(Error::InvalidObjectDef("Picklist field should have business type of either MultiselectPicklist or Picklist"))?,
            }
    }

    Ok(())
}

/// Generate the various reference strings to link the object definition picklist fields to the
/// definition of the picklist.
fn gen_picklist_references(erc: &str) -> Option<Vec<ObjectOrReference<Schema>>> {
    Some(vec![
        ObjectOrReference::Ref {
            ref_path: schema_refpath_picklist_key(erc),
        },
        ObjectOrReference::Ref {
            ref_path: schema_refpath_piclist_item(erc),
        },
    ])
}
