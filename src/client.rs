use std::{collections::HashMap, error::Error, time::Duration};

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderName, HeaderValue, COOKIE},
};
use url::Url;

pub enum SQLandBodyType {
    JSON,
    FORM,
    RAW,
}

pub struct SQLandSettings {
    pub method: Option<String>,
    pub headers: Option<Vec<String>>,
    pub cookies: Option<Vec<String>>,
    pub params: Option<Vec<String>>,
    pub data: Option<Vec<String>>,
    pub body_type: Option<SQLandBodyType>,
}

pub struct SQLand {
    method: String,
    headers: Vec<String>,
    cookies: Vec<String>,
    params: Vec<String>,
    data: Vec<String>,
    body_type: SQLandBodyType,

    url: String,
}

impl SQLand {
    pub fn new(url: String, settings: Option<SQLandSettings>) -> Self {
        let settings = settings.unwrap_or(SQLandSettings {
            body_type: None,
            cookies: None,
            data: None,
            headers: None,
            method: None,
            params: None,
        });

        SQLand {
            method: settings.method.unwrap_or("GET".to_string()),
            body_type: SQLandBodyType::RAW,
            headers: settings.headers.unwrap_or_default(),
            cookies: settings.cookies.unwrap_or_default(),
            params: settings.params.unwrap_or_default(),
            data: settings.data.unwrap_or_default(),
            url,
        }
    }

    pub fn create_headers(&self) -> Result<HeaderMap, Box<dyn Error>> {
        let mut headers = HeaderMap::new();

        // Process standard headers
        for header in self.headers.clone() {
            if let Some((key, value)) = header.split_once(": ") {
                let key = key.trim();
                let value = value.trim();
                let header_name = HeaderName::from_bytes(key.as_bytes())?;
                let header_value = HeaderValue::from_str(value)?;
                headers.insert(header_name, header_value);
            }
        }

        // Process cookies
        let mut cookies_str = String::new();
        for cookie in self.cookies.clone().iter() {
            cookies_str.push_str(cookie);
            cookies_str.push_str("; ");
        }
        if !cookies_str.is_empty() {
            cookies_str.pop(); // Remove trailing space
            cookies_str.pop(); // Remove trailing semicolon
        }
        headers.insert(COOKIE, HeaderValue::from_str(&cookies_str)?);

        Ok(headers)
    }

    pub fn create_client(&self) -> Result<Client, Box<dyn Error>> {
        let headers = self.create_headers()?;
        let client = Client::builder()
            .connect_timeout(Duration::from_millis(10000))
            .default_headers(headers)
            .build()?;
        Ok(client)
    }

    pub fn send(&self, payload: String) -> Result<Response, Box<dyn Error>> {
        // Create HTTP client.
        let client = self.create_client()?;

        // Process parameters.
        let mut params: HashMap<_, _> = self
            .params
            .iter()
            .map(|p| (p.as_str(), payload.clone()))
            .collect();

        // Static params.
        for static_param in &self.data {
            let mut split = static_param.split('=');
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            params.insert(key, value.to_string());
        }

        // Send request.
        let method_upper = self.method.to_uppercase();
        let method = method_upper.as_str();

        let request = match method {
            "GET" | "DELETE" | "OPTIONS" => {
                let url = Url::parse_with_params(&self.url, params)?;
                client.request(method.parse()?, url)
            }
            "POST" | "PUT" | "PATCH" => {
                let url = Url::parse(&self.url)?;
                let mut req = client.request(method.parse()?, url);

                match self.body_type {
                    SQLandBodyType::JSON => {
                        req = req.json(&params);
                    }

                    SQLandBodyType::FORM => {
                        req = req.form(&params);
                    }

                    SQLandBodyType::RAW => {
                        req = req.body(serde_json::to_string(&params)?);
                    }
                }

                req
            }
            _ => panic!("Unsupported method"),
        };

        let response = request.send()?;
        Ok(response)
    }
}
