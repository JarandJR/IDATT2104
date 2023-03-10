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
async fn post_code(request: String) -> String {
    println!("Request: {}", request);
    let output = std::process::Command::new("docker")
                .arg("run")
                .arg("-t")
                .arg("--rm")
                .arg("rust:latest")
                .arg("bash")
                .arg("-c")
                .arg(format!(
                    "cargo new program && cd program && printf '{}' > src/main.rs && cargo run",
                    request
                ))
                .output()
                .expect("Failed to execute process");

    let response = std::str::from_utf8(&output.stdout).expect("Could not parse");
    println!("{}", response);
    strip_color_codes(response)
}

#[get("/get_code")]
async fn get_code()  -> Json<String> {
    Json::from(Json(format!("Hello from backend")))
}

fn strip_color_codes(input: &str) -> String {
    let re = regex::Regex::new("\x1B\\[[0-9;]+m").unwrap();
    re.replace_all(input, "").to_string()
}
