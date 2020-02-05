use std::io::Result;

use actix_web::{client::Client, web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> Result<()> {
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

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/*")
        .body("Whoops not found!"))
}

async fn index(_data: (), _client: web::Data<Client>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().content_type("text/*").body("hey nice!"))
}
