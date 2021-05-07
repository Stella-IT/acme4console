use surf;
use serde::{Serialize, Deserialize};
use crate::MEILING_HOST;

#[derive(Serialize)]
struct AccessTokenRequest {
    access_token: String,
}


pub async fn check_access_token_is_valid(token: &str) -> surf::Result<bool> {
    let data = &AccessTokenRequest { access_token: token.into(), };
    let res = surf::post(MEILING_HOST+"/v1/oauth2/tokeninfo").body(
        surf::Body::from_json(data)
    ).await?;

    return Ok(res.status() == 200);
}

