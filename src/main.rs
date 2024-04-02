use actix_cors::Cors;
use tokio;
use actix_web::middleware::Logger;
use actix_web::{dev::Service as _, web, App};
use actix_web::{http, HttpServer};
use env_logger::Env;
use futures_util::future::FutureExt;
mod actions;
mod helpers;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3012")
            // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".localhost"))
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());
                // i want  to log request params and multipart data
                
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .wrap(Logger::default())
            .wrap(Logger::new("Status %s, Took: %Dms"))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .service(actions::root)
            .service(actions::ocr::index)
            .service(actions::ocr::create)
    })
    .bind(("127.0.0.1", 3012))?
    .run()
    .await
}
