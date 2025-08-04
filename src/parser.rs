use std::{collections::HashMap, fs, path::Path};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::request::Request;

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub url: String,
    pub method: String,
    pub header: HashMap<String, String>,
    pub body: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rest {
    pub name: String,
    pub actions: Vec<Action>,
}

pub fn load_rest_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Action>> {
    let path_ref = path.as_ref();
    let file_content = fs::read_to_string(path_ref)
        .with_context(|| format!("Could not read file: {}", path_ref.display()))?;

    let actions: Vec<Action> = serde_yaml::from_str(&file_content)
        .with_context(|| format!("Could not parse YAML from file: {}", path_ref.display()))?;

    Ok(actions)
}

pub async fn parse_to_request(actions: Vec<Action>) -> Vec<Request> {
    let mut requests: Vec<Request> = Vec::new();

    for act in actions {
        let request = Request::new(act.method, act.url, act.body, act.header).await;
        requests.push(request);
    }
    requests
}
