use actix_web::{web, App, HttpResponse, HttpServer};
use percent_encoding::percent_decode_str;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use url::Url as StdUrl;
use urlencoding::encode;
use webbrowser;

use crate::helper::DynError;

const LOGIN_URL: &str = "https://login.salesforce.com/services/oauth2/token";
const REDIRECT_URI: &str = "http://localhost:8000/oauth/callback";

fn get_client_id_and_secret() -> (String, String) {
    let client_id = env::var("SFDC_CLIENT_ID").unwrap();
    let client_secret = env::var("SFDC_CLIENT_SECRET").unwrap();
    (client_id, client_secret)
}

fn get_authorization_url() -> String {
    let (client_id, _) = get_client_id_and_secret();
    let scope = "refresh_token%20full";

    format!(
        "https://login.salesforce.com/services/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}",
        client_id,
        REDIRECT_URI,
        scope
    )
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    instance_url: String,
}

async fn fetch_access_token_with_refresh_token(
    refresh_token: &str,
) -> Result<TokenResponse, DynError> {
    let (client_id, client_secret) = get_client_id_and_secret();

    let client = Client::new();
    let response = client
        .post(LOGIN_URL)
        .form(&[
            ("grant_type", "refresh_token"),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await?;

    let response_text = response.text().await?;

    serde_json::from_str::<TokenResponse>(&response_text).map_err(|err| {
        println!(
            "Failed to parse the response as AccessTokenResponse: {:?}",
            err
        );
        println!("Raw response: {}", response_text);
        Box::new(err) as DynError
    })
}

async fn fetch_access_token(code: &str) -> Result<TokenResponse, DynError> {
    let (client_id, client_secret) = get_client_id_and_secret();

    let client = Client::new();
    let response = client
        .post(LOGIN_URL)
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("redirect_uri", REDIRECT_URI),
            ("code", code),
        ])
        .send()
        .await?;

    let response_text = response.text().await?;

    serde_json::from_str::<TokenResponse>(&response_text).map_err(|err| {
        println!("Failed to parse the response asTokenResponse: {:?}", err);
        println!("Raw response: {}", response_text);
        Box::new(err) as DynError
    })
}

async fn auth_callback(req: actix_web::HttpRequest) -> HttpResponse {
    let query_string = req.query_string();
    let decoded_query_string = percent_decode_str(query_string).decode_utf8_lossy();
    let code = decoded_query_string
        .to_string()
        .replace("code=", "")
        .replace("&state=", "");

    let token_response = fetch_access_token(&code).await.unwrap();

    fs::write("refresh_token.txt", &token_response.refresh_token.unwrap())
        .expect("Unable to write refresh token to file");

    HttpResponse::Ok().body("You can close this window now.")
}

async fn get_access_token_from_refresh_token() -> Result<TokenResponse, DynError> {
    let refresh_token = fs::read_to_string("refresh_token.txt")?;
    let refresh_token = refresh_token.trim().to_string();
    let token_response = fetch_access_token_with_refresh_token(&refresh_token).await?;
    Ok(token_response)
}

pub struct Connection {
    token_response: TokenResponse,
}

impl Connection {
    pub async fn new() -> Result<Self, DynError> {
        // Use the refresh token if it exists
        if Path::new("refresh_token.txt").exists() {
            let token_response = get_access_token_from_refresh_token().await?;

            let token_response = TokenResponse {
                access_token: token_response.access_token,
                instance_url: token_response.instance_url,
                refresh_token: None,
            };

            return Ok(Self {
                token_response: token_response,
            });
        }

        // If the refresh token does not exist, prompt the user to authenticate
        let auth_url = get_authorization_url();

        webbrowser::open(&auth_url)?;

        HttpServer::new(|| App::new().route("/oauth/callback", web::get().to(auth_callback)))
            .bind("localhost:8000")?
            .run()
            .await?;

        let token_response = get_access_token_from_refresh_token().await?;

        let token_response = TokenResponse {
            access_token: token_response.access_token,
            instance_url: token_response.instance_url,
            refresh_token: None,
        };

        Ok(Self {
            token_response: token_response,
        })
    }

    pub async fn call_query(&self, query: &str, open_browser: bool) -> Result<(), DynError> {
        let client = Client::new();
        let mut headers = HeaderMap::new();
        let encoded_query = encode(query);
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.token_response.access_token)
                .parse()
                .unwrap(),
        );
        let url = format!(
            "{}/services/data/v51.0/query/?q={}",
            self.token_response.instance_url, encoded_query
        );
        let query_response = client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if open_browser {
            open_record(&self.token_response, &query_response);
        }
        println!("{}", serde_json::to_string_pretty(&query_response)?);
        Ok(())
    }
}

fn open_record(token_response: &TokenResponse, query_response: &Value) {
    if let Some(record) = query_response["records"].as_array().and_then(|r| r.get(0)) {
        let id = record["Id"].as_str().unwrap_or("");
        let instance_url = &token_response.instance_url;
        let url = format!("{}{}", instance_url, "/".to_owned() + id);
        if let Err(e) = webbrowser::open(&url) {
            println!("Failed to open URL: {}", e);
        }
    }
}
