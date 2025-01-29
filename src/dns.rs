use crate::Result;
use serde::{Deserialize, Serialize};

pub async fn query(domain: &str, r#type: &str) -> Result<impl Iterator<Item = String>> {
    Ok(reqwest::get(format!(
        "https://dns.google/resolve?type={type}&name={domain}"
    ))
    .await?
    .json::<Response>()
    .await?
    .Answer
    .into_iter()
    .map(|Answer { data }| data))
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Response {
    Answer: Vec<Answer>,
}

#[derive(Serialize, Deserialize)]
struct Answer {
    data: String,
}
