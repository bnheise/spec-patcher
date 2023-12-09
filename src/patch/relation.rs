use liferay_object::models::ObjectDefinition;
use oas::OpenAPIV3;

use crate::object;

/// Liferay's OOTB support for relations in open api specs is limited to a stub,
/// which is not useful for generating types. For this reason, we remove the schemas
/// entirely, expecting instead that the user has already generated the necessary
/// types and that they will be available as part of the existing output file
#[allow(unused)]
pub fn remove_relations(object_def: &ObjectDefinition, spec: &mut OpenAPIV3) {
    // TODO: do we need this?
    let mut relation_names = object::extract::parent_object_names(object_def);
    let child_object_names = object::extract::child_object_names(object_def);
    relation_names.extend(child_object_names);
    relation_names.into_iter().for_each(|name| {
        spec.components
            .as_mut()
            .map(|comp| comp.schemas.as_mut().map(|schemas| schemas.remove(&name)));
    })
}
