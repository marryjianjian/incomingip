use actix_web::{get, App, HttpRequest, HttpServer, Responder};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let url = req.url_for_static("ip").unwrap();

    url.to_string()
}

#[get("/ip")]
async fn ip(req: HttpRequest) -> impl Responder {
    if let Some(rip) = req.headers().get("X-Real-Ip") {
        format!("{}", rip.to_str().unwrap_or(""))
    } else {
        "No ip found.".to_owned()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8081;
    println!("listen at {host}:{port}");
    HttpServer::new(|| App::new().service(index).service(ip))
        .bind((host, port))?
        .run()
        .await
}
