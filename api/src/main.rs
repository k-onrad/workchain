use actix_web::{get, middleware, web, App, HttpRequest, HttpServer};
use blockchain::blockchain::Blockchain;
use pretty_env_logger;
use std::env;
use std::io;

#[get("/{name}")]
async fn index(req: HttpRequest, name: web::Path<String>) -> String {
    println!("REQ: {:?}", req);
    format!("Hello: {}!\r\n", name)
}

#[get("/api/blocks")]
async fn get_blocks(_req: HttpRequest) -> String {
    unimplemented!();
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    pretty_env_logger::init();

    let bc = Blockchain::new();

    HttpServer::new(|| {
        App::new()
            .data(bc)
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_blocks)
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await
}
