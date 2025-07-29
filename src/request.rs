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
}

impl Request {
    pub async fn new(
        method: String,
        endpoint: String,
        body: HashMap<String, String>,
        header: HashMap<String, String>,
    ) -> Self {
        Self {
            method,
            endpoint,
            body,
            header,
            client: Client::new(),
        }
    }

    pub async fn run(&self) -> Result<String, Error> {
        let method = self.method.clone();
        match method.as_str() {
            "GET" => self.get().await,
            "POST" => self.post().await,
            "DELETE" => self.delete().await,
            _ => panic!("Method not allowed"),
        }
    }
    pub async fn get(&self) -> Result<String, Error> {
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
    pub async fn post(&self) -> Result<String, Error> {
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
    pub async fn delete(&self) -> Result<String, Error> {
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
    pub async fn put(self) -> Result<String, Error> {
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
    pub async fn patch(self) -> Result<String, Error> {
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
