use convert_case::{Case, Casing};

pub fn schema_refpath_picklist_key(picklist_erc: &str) -> String {
    let new_case = picklist_erc.to_case(Case::Camel); // TODO: need to make this configurable?
    format!("#/components/schemas/{new_case}Key")
}

pub fn schema_refpath_piclist_item(picklist_erc: &str) -> String {
    let new_case = picklist_erc.to_case(Case::Camel); // TODO: need to make this configurable?
    format!("#/components/schemas/{new_case}Item")
}

pub fn fieldname_enum_name(object_name: &str) -> String {
    format!("{object_name}FieldName")
}

pub fn erc_enum_name(object_name: &str) -> String {
    format!("{object_name}FieldErc")
}

pub fn relation_enum_name(object_name: &str) -> String {
    format!("{object_name}RelationFieldName")
}

pub fn nested_field_enum_name(object_name: &str) -> String {
    format!("{object_name}NestedFieldName")
}
