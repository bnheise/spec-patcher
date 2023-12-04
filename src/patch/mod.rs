use crate::{config::Config, load::MetaData};
use openapi::v3_0::Spec;

use self::picklist::gen_picklist_enums;

pub mod error;
mod picklist;

pub fn patch(_config: &Config, metadata: MetaData) -> Spec {
    let mut spec = metadata.spec;
    if let Some(picklists) = metadata.picklists {
        gen_picklist_enums(picklists, &mut spec);
    }

    spec
}
