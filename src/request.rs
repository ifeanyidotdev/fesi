use std::collections::HashMap;

use reqwest::{Client, Error, header::HeaderMap};

pub struct Request {
    pub method: String,
    pub endpoint: String,
    pub body: HashMap<String, String>,
    pub header: HashMap<String, String>,
}

impl Request {
    pub async fn get(&self) -> Result<String, Error> {
        let client: Client = Client::new();
        let mut headers = HeaderMap::new();

        let response = client
            .get(self.endpoint.clone())
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }
}
