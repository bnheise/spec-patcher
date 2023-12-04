use openapi::v3_0::Spec;
use serde_json::Value;
use std::{fs, io::Write};
use thiserror::Error;

use crate::config::sub::output::Output;

pub fn output(config: &Output, spec: &Spec) -> Result<(), OutputError> {
    let output = serde_json::to_string_pretty(&spec)?;
    if let Some(path) = &config.output {
        let spec = if path.exists() {
            let out_file = fs::read_to_string(path)?;
            let mut value = serde_json::to_value(out_file)?;
            let spec = serde_json::to_value(spec)?;
            merge(&mut value, spec);
            serde_json::to_string_pretty(&value)?
        } else {
            serde_json::to_string_pretty(&spec)?
        };
        fs::write(path, spec)?;
        Ok(())
    } else {
        std::io::stdout().write_all(output.as_bytes())?;
        Ok(())
    }
}

fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b;
}

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Failed to serialize or deserialize spec: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Failed to write open-api spec to file: {0}")]
    Write(#[from] std::io::Error),
}
