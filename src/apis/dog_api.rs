use futures::future::join_all;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ApiResponse {
    message: String,
    status: String,
}

async fn fetch_random_dog_payload() -> Result<ApiResponse, failure::Error> {
    Ok(reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json::<ApiResponse>()
        .await?)
}

pub async fn get_dog_urls(count: usize) -> Result<Vec<String>, failure::Error> {
    // Creating an 'empty' vector here so it can be filled with futures
    // Is there a better way to do this?
    Ok(join_all(
        vec![0; count]
            .into_iter()
            .map(|_x| fetch_random_dog_payload()),
    )
    .await
    .into_iter()
    .map(|result| match result {
        Ok(payload) => payload.message,
        Err(error) => error.to_string(),
    })
    .collect())
}
