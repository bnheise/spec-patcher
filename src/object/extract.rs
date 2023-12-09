use liferay_object::models::{object_field::BusinessType, ObjectDefinition, ObjectField};

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

pub fn picklist_fields(object_def: &ObjectDefinition) -> Vec<&ObjectField> {
    object_def
        .object_fields
        .as_ref()
        .map(|fields| {
            fields
                .iter()
                .filter(|&field| field.list_type_definition_external_reference_code.is_some())
                .filter(|&field| {
                    !field
                        .external_reference_code
                        .as_deref()
                        .unwrap_or("")
                        .is_empty()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub fn object_name(object_def: &ObjectDefinition) -> Option<&str> {
    object_def.name.as_deref()
}

/// Get all fields with bu
pub fn attachment_fields(object_def: &ObjectDefinition) -> Vec<&ObjectField> {
    object_def
        .object_fields
        .as_ref()
        .map(|object_fields| {
            object_fields
                .iter()
                .filter(|&field| {
                    field
                        .business_type
                        .map(|bus_type| bus_type == BusinessType::Attachment)
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// Get all fields which are not relationships or system fields
pub fn standard_fields(object_def: &ObjectDefinition) -> Vec<&ObjectField> {
    object_def
        .object_fields
        .as_ref()
        .map(|fields| {
            fields
                .iter()
                .filter(|field| {
                    !field.system.unwrap_or_default()
                        && field
                            .business_type
                            .filter(|bus_type| *bus_type != BusinessType::Relationship)
                            .is_none()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// Get vectors of all field names for child and parent relationships
pub fn relationship_fields(object_def: &ObjectDefinition) -> Vec<&ObjectField> {
    object_def
        .object_fields
        .as_ref()
        .map(|fields| {
            fields
                .iter()
                .filter(|field| {
                    !field.system.unwrap_or_default()
                        && field
                            .business_type
                            .filter(|bus_type| *bus_type == BusinessType::Relationship)
                            .is_some()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}
