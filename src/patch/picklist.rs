use crate::{config::Config, error::Reporter};

use super::Error;
use convert_case::Casing;
use list_type::models::ListTypeDefinition;
use openapi::v3_0::{Components, ObjectOrReference, Schema, Spec};
use std::collections::BTreeMap;

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
