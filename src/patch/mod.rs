use crate::{config::Config, load::MetaData};
use openapi::v3_0::Spec;

use self::{picklist::gen_picklist_enums, relation::remove_relations};

mod error;
mod picklist;
mod relation;
pub use error::Error;

pub fn patch(config: &Config, metadata: MetaData) -> Spec {
    let mut spec = metadata.spec;

    if let Some(picklists) = metadata.picklists {
        gen_picklist_enums(config, picklists, &mut spec);
    }

    if let Some(ref object_def) = metadata.object_def {
        remove_relations(object_def, &mut spec);
    }

    spec
}
