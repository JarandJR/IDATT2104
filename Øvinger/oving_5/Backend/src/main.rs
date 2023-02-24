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
    println!("Request: {}", request);
    let command = format!("echo {}", &request);
    let command = "ping 8.8.8.8";
    let output = std::process::Command::new("cmd")
                .args(["/C", &command])
                .output()
                .expect("failed to execute process");

    let response = std::str::from_utf8(&output.stdout).expect("Could not parse");
    println!("{}", response);
    
    Json::from(Json(response.to_string()))
}

#[get("/get_code")]
async fn get_code()  -> Json<String> {
    Json::from(Json(format!("Hello from backend")))
}