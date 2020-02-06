use actix_web::{client::Client, web, App, Error, HttpResponse, HttpServer};

mod apis;

use apis::dog_api;

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    let endpoint = "127.0.0.1:7878";

    HttpServer::new(|| {
        App::new()
            .data(Client::default())
            .service(web::resource("/").route(web::get().to(index)))
            .default_service(web::route().to(not_found))
    })
    .bind(endpoint)?
    .run()
    .await
}

async fn not_found() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("Whoops not found!"))
}

fn create_img_tag(url: String) -> String {
    return format!("<img src=\"{}\" />", url);
}

async fn index(_data: (), _client: web::Data<Client>) -> Result<HttpResponse, failure::Error> {
    let dog_urls_result = dog_api::get_dog_urls(5).await;

    match dog_urls_result {
        Ok(dog_urls) => {
            let dog_imgs: Vec<String> = dog_urls.into_iter().map(create_img_tag).collect();
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(dog_imgs.join("")))
        }
        Err(error) => Ok(HttpResponse::from_error(actix_http::error::Error::from(
            error,
        ))),
    }
}
