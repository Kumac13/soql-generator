use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::result::Result;
use urlencoding::encode;

use crate::helper::DynError;

const LOGIN_URL: &str = "https://login.salesforce.com/services/oauth2/token";

#[derive(Debug, Deserialize, Serialize)]
struct LoginRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LoginResponse {
    access_token: String,
    instance_url: String,
}

pub struct Connection {
    login_response: LoginResponse,
}

impl Connection {
    pub async fn new() -> Result<Self, DynError> {
        let client_id = env::var("SFDC_CLIENT_ID")?;
        let client_secret = env::var("SFDC_CLIENT_SECRET")?;
        let username = env::var("SFDC_USERNAME")?;
        let password = env::var("SFDC_USERPASSWORD")?;

        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        let request = LoginRequest {
            grant_type: "password".to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        };

        let response = client
            .post(LOGIN_URL)
            .headers(headers)
            .form(&request)
            .send()
            .await?
            .json::<LoginResponse>()
            .await?;

        Ok(Self {
            login_response: response,
        })
    }

    pub async fn call_query(&self, query: &str, open_browser: bool) -> Result<(), DynError> {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        let encoded_query = encode(query);
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.login_response.access_token)
                .parse()
                .unwrap(),
        );
        let url = format!(
            "{}/services/data/v51.0/query/?q={}",
            self.login_response.instance_url, encoded_query
        );
        let query_response = client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if open_browser {
            open_record(&self.login_response, &query_response);
        }

        println!("{}", serde_json::to_string_pretty(&query_response)?);
        Ok(())
    }
}

fn open_record(login_response: &LoginResponse, query_response: &Value) {
    if let Some(record) = query_response["records"].as_array().and_then(|r| r.get(0)) {
        let id = record["Id"].as_str().unwrap_or("");
        let instance_url = &login_response.instance_url;
        let url = format!("{}{}", instance_url, "/".to_owned() + id);
        if let Err(e) = webbrowser::open(&url) {
            println!("Failed to open URL: {}", e);
        }
    }
}
