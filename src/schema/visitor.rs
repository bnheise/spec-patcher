use std::collections::BTreeMap;

use oas::{
    Components, ExternalDocumentation, Info, OpenAPIV3, PathItem, Referenceable, Schema, Server,
    Tag,
};

pub trait OpenApiVisitorMut {
    fn visit_spec(&mut self, _: &mut OpenAPIV3) {}
    fn visit_info(&mut self, _: &mut Info) {}
    fn visit_server(&mut self, _: &mut Server) {}
    fn visit_path(&mut self, _: &(&String, &mut PathItem)) {}
    fn visit_components(&mut self, _: &mut Components) {}
    fn visit_tag(&mut self, _: &mut Tag) {}
    fn visit_external_doc(&mut self, _: &mut ExternalDocumentation) {}
    fn visit_schemas(&mut self, _: &mut Option<BTreeMap<String, Referenceable<Schema>>>) {}
    // fn visit_extension<T>(&mut self, _: &mut Extensions, _: T) {}
    // fn visit_contact(&mut self, _: &mut Option<Contact>) {}
    // fn visit_license(&mut self, _: &mut Option<License>) {}
    // fn visit_server_variables(&mut self, _: &mut Option<BTreeMap<String, ServerVariable>>) {}
    // fn visit_operation_get(&mut self, _: &mut Option<Operation>) {}
    // fn visit_operation_put(&mut self, _: &mut Option<Operation>) {}
    // fn visit_operation_post(&mut self, _: &mut Option<Operation>) {}
    // fn visit_operation_delete(&mut self, _: &mut Option<Operation>) {}
    // fn visit_operation_options(&mut self, _: &mut Option<Operation>) {}
    // fn visit_operation_head(&mut self, _: &mut Option<Operation>) {}
    // fn visit_operation_patch(&mut self, _: &mut Option<Operation>) {}
    // fn visit_operation_trace(&mut self, _: &mut Option<Operation>) {}
    // fn visit_parameters(&mut self, _: &mut Option<Vec<ObjectOrReference<Parameter>>>) {}
    // fn visit_request_body(&mut self, _: &mut Option<Vec<ObjectOrReference<Parameter>>>) {}
    // fn visit_responses(&mut self, _: &mut BTreeMap<String, Response>) {}
    // fn visit_callbacks(&mut self, _: &mut Option<BTreeMap<String, Callback>>) {}
    // fn visit_location(&mut self, _: &mut Location) {}
    // fn visit_schema(&mut self, _: &mut Option<Schema>) {}
    // fn visit_parameter_style(&mut self, _: &mut Option<ParameterStyle>) {}
    // fn visit_schema_type(&mut self, _: &mut Option<SchemaType>) {}
    // fn visit_schema_format(&mut self, _: &mut Option<SchemaFormat>) {}
    // fn visit_items(&mut self, _: &mut Option<Box<Schema>>) {}
    // fn visit_properties(&mut self, _: &mut Option<BTreeMap<String, Schema>>) {}
    // fn visit_additional_properties(
    //     &mut self,
    //     _: &mut Option<BooleanObjectOrReference<Box<Schema>>>,
    // ) {
    // }
    // fn visit_example(&mut self, _: &mut Option<serde_json::value::Value>) {}
    // fn visit_all_of(&mut self, _: &mut Option<Vec<ObjectOrReference<Schema>>>) {}
    // fn visit_one_of(&mut self, _: &mut Option<Vec<ObjectOrReference<Schema>>>) {}
    // fn visit_any_of(&mut self, _: &mut Option<Vec<ObjectOrReference<Schema>>>) {}
    // fn visit_not(&mut self, _: &mut Option<Vec<ObjectOrReference<Schema>>>) {}
    // fn visit_xml(&mut self, _: &mut Option<XmlObject>) {}
    // fn visit_headers(&mut self, _: &mut Option<BTreeMap<String, ObjectOrReference<Header>>>) {}
    // fn visit_content(&mut self, _: &mut Option<BTreeMap<String, MediaType>>) {}
    // fn visit_links(&mut self, _: &mut Option<BTreeMap<String, ObjectOrReference<Link>>>) {}
}
