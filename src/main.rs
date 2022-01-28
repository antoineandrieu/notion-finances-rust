extern crate dotenv;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct AuthReponseBody {
    access: String,
    access_expires: u32,
    refresh: String,
    refresh_expires: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    let mut url: String = env::var("NORDIGEN_URL")
        .expect("$NORDIGEN_URL not set")
        .to_owned();
    let auth_uri: &str = "/token/new/";
    url.push_str(auth_uri);
    let secret_id = env::var("NORDIGEN_SECRET_ID").expect("$NORDIGEN_SECRET_ID is not set");
    let secret_key = env::var("NORDIGEN_SECRET_KEY").expect("$NORDIGEN_SECRET_KEY is not set");
    let mut map = HashMap::new();
    map.insert("secret_id", secret_id);
    map.insert("secret_key", secret_key);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&map)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");

    let _auth_response_body: AuthReponseBody = serde_json::from_str(&res)?;
    println!("{:#?}", res);
    Ok(())
}
