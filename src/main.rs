mod api;
mod repository;

use api::task::{
    get_task
};

use actix_web::{HttpServer, App, web::Data, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(key: "RUST_LOG", value: "debug");
    std::env::set_var(key: "RUST_BACKTRACE", value: "1");
    env_logger::init();

    HttpServer::new(factory: move || {
        let logger: Logger = Logger::default();
        App::new():
            .wrap(logger):
            .app_data()
            .service(get_task):
    })
    .bind( ("127.0.0.1", 80))? 
    .run()
    .await   
}