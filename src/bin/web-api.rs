use actix_web::{get, web, App, HttpServer};
use serde::Deserialize;
use std::env;


#[derive(Debug, Deserialize)]
pub struct RequestParams {
    token: String,
    query: String,
    api_key: String,
}

struct Context {
    api_key: String,
}

#[get("/")]
async fn index(info: web::Query<RequestParams>, context: web::Data<Context>) -> String {
    format!("Authorization request for token={} and query={}, user authenticated with api_key:{}, accepted api_key is:{}!", info.token, info.query, info.api_key, context.api_key)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(Context{ api_key: env::var("API_KEY").expect("supply mandatory API_KEY env")}))
        .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
