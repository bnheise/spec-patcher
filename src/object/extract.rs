use liferay_object::models::{object_field::BusinessType, ObjectDefinition};

pub fn picklist_ercs(object_def: &ObjectDefinition) -> Vec<String> {
    object_def
        .object_fields
        .as_ref()
        .map(|fields| {
            fields
                .iter()
                .filter_map(|field| {
                    field
                        .list_type_definition_external_reference_code
                        .to_owned()
                })
                .collect::<Vec<String>>()
        })
        .unwrap_or_default()
}

pub fn parent_object_names(object_def: &ObjectDefinition) -> Vec<String> {
    object_def
        .object_fields
        .as_ref()
        .map(|fields| {
            let names = fields
                .iter()
                .filter(|&field| {
                    field
                        .business_type
                        .map(|bus_type| Some(bus_type == BusinessType::Relationship))
                        .unwrap_or_default()
                        .is_some()
                })
                .flat_map(|field| &field.object_field_settings)
                .flatten()
                .filter(|setting| {
                    setting.name.as_deref().unwrap_or("") == "objectDefinition1ShortName"
                })
                .filter_map(|setting| setting.value.as_ref())
                .map(|value| value.to_string())
                .collect::<Vec<_>>();

            names
        })
        .unwrap_or_default()
}

pub fn child_object_names(object_def: &ObjectDefinition) -> Vec<String> {
    object_def
        .object_relationships
        .as_ref()
        .map(|relation| {
            relation
                .iter()
                .flat_map(|relation| relation.object_definition_name2.to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}
