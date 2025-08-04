use std::{collections::HashMap, str::FromStr};

use reqwest::{
    Client, Error,
    header::{HeaderMap, HeaderName, HeaderValue},
};

#[derive(Debug)]
pub struct Request {
    client: Client,
    pub method: String,
    pub endpoint: String,
    pub body: HashMap<String, String>,
    pub header: HashMap<String, String>,
    pub name: Option<String>,
    pub save_response: bool,
}

impl Request {
    pub async fn new(
        method: String,
        endpoint: String,
        body: HashMap<String, String>,
        header: HashMap<String, String>,
        name: Option<String>,
        save_response: bool,
    ) -> Self {
        Self {
            method,
            endpoint,
            body,
            header,
            client: Client::new(),
            name,
            save_response,
        }
    }

    pub async fn run(&self) -> anyhow::Result<String, Error> {
        let method = self.method.clone();
        match method.as_str() {
            "GET" => self.get().await,
            "POST" => self.post().await,
            "PUT" => self.put().await,
            "DELETE" => self.delete().await,
            _ => panic!("Method not allowed"),
        }
    }

    pub async fn get(&self) -> anyhow::Result<String, Error> {
        let mut headers = HeaderMap::new();

        let request_h = self.header.clone();

        for (key, value) in request_h.into_iter() {
            let header_name = HeaderName::from_str(&key).expect("Invalid header name");
            let header_value = HeaderValue::from_str(&value).expect("Invalid header value");
            headers.insert(header_name, header_value);
        }
        let response = self
            .client
            .get(self.endpoint.clone())
            .headers(headers)
            .send()
            .await?;

        let result = response.text().await?;

        Ok(result)
    }
    pub async fn post(&self) -> anyhow::Result<String, Error> {
        let mut headers = HeaderMap::new();

        let request_h = self.header.clone();

        for (key, value) in request_h.into_iter() {
            let header_name = HeaderName::from_str(&key).expect("Invalid header name");
            let header_value = HeaderValue::from_str(&value).expect("Invalid header value");
            headers.insert(header_name, header_value);
        }
        let response = self
            .client
            .post(self.endpoint.clone())
            .headers(headers)
            .json(&self.body)
            .send()
            .await?;

        let result = response.text().await?;
        Ok(result)
    }
    pub async fn delete(&self) -> anyhow::Result<String, Error> {
        let mut headers = HeaderMap::new();
        let request_h = self.header.clone();

        for (key, value) in request_h.into_iter() {
            let header_name = HeaderName::from_str(&key).expect("Invalid header name");
            let header_value = HeaderValue::from_str(&value).expect("Invalid header value");
            headers.insert(header_name, header_value);
        }
        let response = self
            .client
            .delete(self.endpoint.clone())
            .headers(headers)
            .send()
            .await?;

        let result = response.text().await?;
        Ok(result)
    }
    pub async fn put(&self) -> anyhow::Result<String, Error> {
        let mut headers = HeaderMap::new();

        let request_h = self.header.clone();

        for (key, value) in request_h.into_iter() {
            let header_name = HeaderName::from_str(&key).expect("Invalid header name");
            let header_value = HeaderValue::from_str(&value).expect("Invalid header value");
            headers.insert(header_name, header_value);
        }
        let response = self
            .client
            .put(self.endpoint.clone())
            .headers(headers)
            .json(&self.body)
            .send()
            .await?;

        let result = response.text().await?;
        Ok(result)
    }
    pub async fn patch(&self) -> Result<String, Error> {
        let mut headers = HeaderMap::new();

        let request_h = self.header.clone();

        for (key, value) in request_h.into_iter() {
            let header_name = HeaderName::from_str(&key).expect("Invalid header name");
            let header_value = HeaderValue::from_str(&value).expect("Invalid header value");
            headers.insert(header_name, header_value);
        }
        let response = self
            .client
            .patch(self.endpoint.clone())
            .headers(headers)
            .json(&self.body)
            .send()
            .await?;

        let result = response.text().await?;
        Ok(result)
    }
}
