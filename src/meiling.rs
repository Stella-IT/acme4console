use surf;
use serde::{Serialize, Deserialize};
use crate::{CLIENT_ID, MEILING_HOST};

#[derive(Serialize)]
struct AccessTokenRequest {
    access_token: String,
}

#[derive(Serialize)]
struct RequestAccessTokenViaRefreshToken {
    client_id: String,
    grant_type: String,
    refresh_token: String,
}

#[derive(Serialize)]
struct RequestAccessTokenViaAuthorizationCode {
    client_id: String,
    grant_type: String,
    authorization_code: String,
}

#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: u32,
    scope: String,
    token_type: String,
}

#[derive(Deserialize)]
struct AuthorizationCodeResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u32,
    scope: String,
    token_type: String,
}


pub async fn check_access_token_is_valid(token: &str) -> surf::Result<bool> {
    let data = &AccessTokenRequest { access_token: token.into(), };
    let res = surf::post(MEILING_HOST+"/v1/oauth2/tokeninfo").body(
        surf::Body::from_json(data)
    ).await?;

    return Ok(res.status() == 200);
}

pub async fn issue_access_token_via_refresh_token(token: &str) -> surf::Result<String> {
    let data = &RequestAccessTokenViaRefreshToken {
        client_id: CLIENT_ID.into(),
        grant_type: "refresh_token".into(),
        refresh_token: token.into(),
    };

    let mut res = surf::post(MEILING_HOST+"/v1/oauth2/token").body(
        surf::Body::from_json(data)
    ).await?;

    let json: AccessTokenRequest = res.body_json().await?;

    return Ok(json.access_token);
}

