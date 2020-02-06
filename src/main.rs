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
    format!("<img src=\"{}\" style=\"max-width: 300px\" />", url)
}

async fn index(_data: (), _client: web::Data<Client>) -> Result<HttpResponse, failure::Error> {
    match dog_api::get_dog_urls(5).await {
        Ok(dog_urls) => Ok(HttpResponse::Ok().content_type("text/html").body(
            dog_urls
                .into_iter()
                .map(create_img_tag)
                .collect::<Vec<String>>()
                .join(""),
        )),
        Err(error) => Ok(HttpResponse::from_error(actix_http::error::Error::from(
            error,
        ))),
    }
}
