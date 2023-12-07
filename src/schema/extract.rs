use openapi::v3_0::{ObjectOrReference, Schema, Spec};

pub fn component_schema_mut<'a>(
    spec: &'a mut Spec,
    schema_name: &str,
) -> Option<&'a mut ObjectOrReference<Schema>> {
    spec.components
        .as_mut()
        .and_then(|comp| {
            comp.schemas
                .as_mut()
                .map(|schemas| schemas.get_mut(schema_name))
        })
        .flatten()
}

pub fn object_schema_mut<'a>(object_name: &str, spec: &'a mut Spec) -> Option<&'a mut Schema> {
    let object_or_ref = match component_schema_mut(spec, object_name) {
        Some(object_or_ref) => object_or_ref,
        None => return None,
    };

    match object_or_ref {
        ObjectOrReference::Object(schema) => Some(schema),
        ObjectOrReference::Ref { .. } => None,
    }
}
