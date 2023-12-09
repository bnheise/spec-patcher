use oas::{Components, ExternalDocumentation, Info, OpenAPIV3, PathItem, Server, Tag};

use super::visitor::OpenApiVisitorMut;
use std::fmt::Debug;

pub struct SpecWalker {
    visitors: Vec<Box<dyn OpenApiVisitorMut>>,
}

impl Debug for SpecWalker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpecWalker")
            .field("visitors", &format!("[{} visitors]", self.visitors.len()))
            .finish()
    }
}

impl SpecWalker {
    pub fn walk(&mut self, spec: &mut OpenAPIV3) {
        self.walk_spec(spec);
    }

    fn walk_spec(&mut self, spec: &mut OpenAPIV3) {
        self.visitors
            .iter_mut()
            .for_each(|visitor| visitor.visit_spec(spec));

        if let Some(components) = spec.components.as_mut() {
            self.walk_components(components);
        }

        self.walk_info(&mut spec.info);

        if let Some(servers) = spec.servers.as_mut() {
            servers
                .iter_mut()
                .for_each(|server| self.walk_server(server))
        };

        spec.paths.iter_mut().for_each(|path| self.walk_path(&path));

        if let Some(tags) = spec.tags.as_mut() {
            tags.iter_mut().for_each(|tag| self.walk_tag(tag))
        }

        if let Some(docs) = spec.external_docs.as_mut() {
            self.walk_external_doc(docs);
        }
    }

    fn walk_components(&mut self, components: &mut Components) {
        self.visitors
            .iter_mut()
            .for_each(|visitor: &mut Box<dyn OpenApiVisitorMut>| {
                visitor.visit_components(components)
            });
    }

    fn walk_info(&mut self, info: &mut Info) {
        self.visitors
            .iter_mut()
            .for_each(|visitor: &mut Box<dyn OpenApiVisitorMut>| visitor.visit_info(info));
    }

    fn walk_server(&mut self, servers: &mut Server) {
        self.visitors
            .iter_mut()
            .for_each(|visitor: &mut Box<dyn OpenApiVisitorMut>| visitor.visit_server(servers));
    }

    fn walk_path(&mut self, path: &(&String, &mut PathItem)) {
        self.visitors
            .iter_mut()
            .for_each(|visitor: &mut Box<dyn OpenApiVisitorMut>| visitor.visit_path(path));
    }

    fn walk_tag(&mut self, tag: &mut Tag) {
        self.visitors
            .iter_mut()
            .for_each(|visitor: &mut Box<dyn OpenApiVisitorMut>| visitor.visit_tag(tag));
    }

    fn walk_external_doc(&mut self, docs: &mut ExternalDocumentation) {
        self.visitors
            .iter_mut()
            .for_each(|visitor: &mut Box<dyn OpenApiVisitorMut>| visitor.visit_external_doc(docs));
    }
}
