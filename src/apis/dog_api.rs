use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ApiResponse {
    message: String,
    status: String,
}

impl ApiResponse {
    pub fn into_message(self) -> String {
        self.message
    }
}

async fn fetch_random_dog_payload() -> Result<ApiResponse, failure::Error> {
    reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json::<ApiResponse>()
        .await
        .map_err(Into::into)
}

pub async fn get_dog_urls(count: usize) -> Result<Vec<String>, failure::Error> {
    use futures::stream::StreamExt;
    use futures::stream::TryStreamExt;
    
    futures::stream::iter(1..=count)
        .then(|_| async move {
            fetch_random_dog_payload()
                .await
                .map(ApiResponse::into_message)
        })
        .try_collect()
        .await
}
