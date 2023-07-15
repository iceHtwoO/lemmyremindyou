use log::debug;
use log::error;
use reqwest::{Error, Url};
use serde_json::{json, Value};
use std::process;

use crate::dto;

pub async fn comment(auth: &str, content: &str, parent_id: i32, post_id: i32) -> Result<(), Error> {
    let body = json!({
        "auth": auth,
        "content": content,
        "parent_id": parent_id,
        "post_id": post_id
    });

    let client = reqwest::Client::new();
    client
        .post(url_builder("api/v3/comment", "").await)
        .json(&body)
        .send()
        .await
        .expect("failed to respond");
    debug!("Sent Comment");
    Ok(())
}

pub async fn mark_read(auth: &str, person_mention_id: i32) -> Result<(), Error> {
    let body = json!({
        "auth": auth,
        "read": true,
        "person_mention_id": person_mention_id
    });

    let client = reqwest::Client::new();
    client
        .post(url_builder("api/v3/user/mention/mark_as_read", "").await)
        .json(&body)
        .send()
        .await?;
    debug!("Read Mention");
    Ok(())
}

pub async fn get_auth_token() -> Result<String, Box<dyn std::error::Error>> {
    let body = json!({
        "username_or_email": std::env::var("BOT_USERNAME").expect("BOT_USERNAME not Set"),
        "password": std::env::var("BOT_PASSWORD").expect("BOT_PASSWORD not Set")
    });

    let client = reqwest::Client::new();
    let out = client
        .post(url_builder("api/v3/user/login", "").await)
        .json(&body)
        .send()
        .await?;

    if out.status() != 200 {
        error!("Authentication Failed with code {}", out.status());
        println!("{}", out.url());
        process::exit(1)
    }

    let out = out.json::<serde_json::Value>().await?;

    debug!("JWT token recived");
    let login_response: dto::Login = serde_json::from_value(out).expect("Err");
    Ok(login_response.jwt.to_string())
}

async fn url_builder(uri: &str, auth_token: &str) -> Url {
    let mut url = String::new();
    let host: String = match std::env::var("HOST"){
        Ok(s) => s,
        Err(_e) => {error!("Host not set"); process::exit(1)},
    };
    url.push_str(&host);
    url.push_str(uri);
    url.push_str("?auth=");
    url.push_str(auth_token);
    match Url::parse(&url) {
        Ok(u) => u,
        Err(_) => {error!("Invalid Domain"); process::exit(1)},
    }
}

pub async fn get_mentions(auth: &str) -> Result<Value, Error> {
    reqwest::get(url_builder("api/v3/user/mention", &auth).await)
        .await?
        .json::<serde_json::Value>()
        .await
}
