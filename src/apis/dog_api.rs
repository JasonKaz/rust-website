use futures::future::join_all;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ApiResponse {
    message: String,
    status: String,
}

async fn fetch_random_dog_payload() -> Result<ApiResponse, failure::Error> {
    let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json::<ApiResponse>()
        .await?;

    Ok(response)
}

pub async fn get_dog_urls(count: usize) -> Result<Vec<String>, failure::Error> {
    let mut i = 0;
    let mut futs = Vec::new();

    while i < count {
        i += 1;

        futs.push(fetch_random_dog_payload());
    }

    Ok(join_all(futs)
        .await
        .into_iter()
        .map(|result| match result {
            Ok(payload) => payload.message,
            Err(error) => error.to_string(),
        })
        .collect())
}
