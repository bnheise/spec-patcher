use crate::{config::Config, load::MetaData};
use openapi::v3_0::Spec;

use self::picklist::gen_picklist_enums;

mod error;
mod picklist;
pub use error::Error;

pub fn patch(config: &Config, metadata: MetaData) -> Spec {
    let mut spec = metadata.spec;
    if let Some(picklists) = metadata.picklists {
        gen_picklist_enums(config, picklists, &mut spec);
    }

    spec
}
