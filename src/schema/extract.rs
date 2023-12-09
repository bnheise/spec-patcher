use oas::{OpenAPIV3, Referenceable, Schema};

pub fn component_schema_mut<'a>(
    spec: &'a mut OpenAPIV3,
    schema_name: &str,
) -> Option<&'a mut Referenceable<Schema>> {
    spec.components
        .as_mut()
        .and_then(|comp| {
            comp.schemas
                .as_mut()
                .map(|schemas| schemas.get_mut(schema_name))
        })
        .flatten()
}

pub fn object_schema_mut<'a>(object_name: &str, spec: &'a mut OpenAPIV3) -> Option<&'a mut Schema> {
    let object_or_ref = match component_schema_mut(spec, object_name) {
        Some(object_or_ref) => object_or_ref,
        None => return None,
    };

    match object_or_ref {
        Referenceable::Data(schema) => Some(schema),
        Referenceable::Reference(..) => None,
    }
}
