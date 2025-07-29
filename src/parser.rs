use std::{collections::HashMap, fs, path::Path};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Method {
    #[serde(alias = "post", alias = "POST")]
    Post,
    #[serde(alias = "get", alias = "GET")]
    Get,
    #[serde(alias = "patch", alias = "PATCH")]
    Patch,
    #[serde(alias = "put", alias = "PUT")]
    Put,
    #[serde(alias = "delete", alias = "DELETE")]
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub url: String,
    pub method: Method,
    pub header: Vec<HashMap<String, String>>,
    pub body: Vec<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rest {
    endpoints: Vec<Endpoint>,
}

impl Rest {
    pub fn load_rest_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path_ref = path.as_ref();
        let file_content = fs::read_to_string(path_ref)
            .with_context(|| format!("Could not read config file: {}", path_ref.display()))?;

        let rest: Rest = serde_yaml::from_str(&file_content)
            .with_context(|| format!("Could not parse YAML from file: {}", path_ref.display()))?;

        Ok(rest)
    }
    /// Loads the configuration from a string.
    pub fn load_from_str(content: &str) -> anyhow::Result<Self> {
        let rest: Rest =
            serde_yaml::from_str(content).context("Could not parse YAML from string")?;
        Ok(rest)
    }
}
