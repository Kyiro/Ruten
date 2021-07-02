use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

pub const CLIENT_ID: &str = "3446cd72694c4a4485d81b77adbb2141";
pub const SECRET: &str = "9209d4a5e25a457fb9b07489d313b41a";

pub const OAUTH_TOKEN: &str =
    "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";

pub fn auth_code_url() -> String {
    [
        "https://epicgames.com/id/api/redirect?clientId=",
        CLIENT_ID,
        "^&responseType=code",
    ]
    .join("")
}

#[derive(Deserialize, Serialize)]
pub struct Access {
    pub access_token: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[tokio::main]
pub async fn auth_code(code: &str) -> Result<Access, Box<dyn StdError>> {
    let client = reqwest::Client::new();

    let req = client
        .post(OAUTH_TOKEN)
        .basic_auth(CLIENT_ID, Some(SECRET))
        .form(&[("grant_type", "authorization_code"), ("code", code)])
        .send()
        .await?;

    Ok(req.json().await?)
}
