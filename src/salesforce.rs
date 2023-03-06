use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::error::Error;
use tokio::runtime::Runtime;

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

struct QueryRequest {
    q: String,
}

async fn login(
    client_id: &str,
    client_secret: &str,
    username: &str,
    password: &str,
) -> Result<LoginResponse, Box<dyn std::error::Error>> {
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

    Ok(response)
}

async fn call_query(
    access_token: &str,
    instance_url: &str,
    query: &str,
) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", access_token).parse().unwrap(),
    );
    let request = QueryRequest {
        q: query.to_string(),
    };
    let url = format!(
        "{}/services/data/v51.0/query/?q={}",
        instance_url, request.q
    );
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(response)
}

pub async fn run(query: &str) -> Result<(), Box<dyn Error>> {
    let client_id = env::var("SFDC_CLIENT_ID")?;
    let client_secret = env::var("SFDC_CLIENT_SECRET")?;
    let username = env::var("SFDC_USERNAME")?;
    let password = env::var("SFDC_USERPASSWORD")?;
    let login_response = login(&client_id, &client_secret, &username, &password).await?;
    let query_response = call_query(
        &login_response.access_token,
        &login_response.instance_url,
        query,
    )
    .await?;
    println!("{}", serde_json::to_string_pretty(&query_response)?);
    Ok(())
}
