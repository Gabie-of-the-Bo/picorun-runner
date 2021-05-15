mod runner;
mod fsetup;
mod preproc;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use actix_web::{web, App, HttpServer};

use runner::*;
use fsetup::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init_timed();

    HttpServer::new(|| {
        App::new()
            .route("python/write-prep", web::post().to(write_preparation_python))
            .route("python/write-exec", web::post().to(write_execution_python))
            .route("python/write-code", web::post().to(write_code_python))
            .route("python/execute", web::get().to(execute_python))
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}