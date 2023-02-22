use actix_web::{get, post, web::Json, App, HttpServer};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(post_code)
            .service(get_code)
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}

#[post("/post_code")]
async fn post_code(request: String) -> Json<String> {
    Json::from(Json(request))
}

#[get("/get_code")]
async fn get_code()  -> Json<String> {
    Json::from(Json(format!("Hello from backend")))
}