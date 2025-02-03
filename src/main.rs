use std::env::{set_var, var_os};

use actix_cors::Cors;
use actix_web::{http::header, main, middleware, web, App, HttpServer};

mod general;
mod something;

#[main]
async fn main() -> std::io::Result<()> {
    // Set environment for logging configuration
    // access logs are printed with the INFO level so
    // ensure it is enabled by default
    if var_os("RUST_LOG").is_none() {
        set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    }
    set_var("RUST_BACKTRACE", "1");
    // Start logging to console
    env_logger::init();

    let bind = ("127.0.0.1", 8080);
    log::info!("staring server at http://{}:{}", &bind.0, &bind.1);
    let _services = actix_web::services![
        general::greet,
        web::resource("/status").route(web::get().to(general::health_check_handler)),
    ];

    // create an application builder
    // The `move` keyword at the front indicates that this
    // closure takes ownership of the variables it uses
    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin("http://localhost:8080/")
            .allowed_methods(vec!["GET", "POST", "HEAD", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT]);

        App::new()
            .configure(general_config)
            // .service(_services)
            .wrap(cors)
            .wrap(middleware::DefaultHeaders::new().add(("X-Version-ID", "1.0")))
            .wrap(middleware::Logger::default())
    };

    // now start the HTTP server
    // HttpServer::new(app).bind(("127.0.0.1", 8080))?.run().await

    // HttpServer::new(app).bind(bind)?.run().await
    //  OR specify the number of workers as below...
    HttpServer::new(app).bind(bind).unwrap().workers(8).run().await
}

pub fn general_config(config: &mut web::ServiceConfig) {
    config.service(general::head_handler);
    config.service(general::greet);
    config.service(general::post_gcd_handler);
    config.service(general::factorize_handler);
    config.service(general::favicon_handler);
    config.service(general::redirect_handler);
    config.service(general::html_handler);
    config.service(general::echo_handler);
    config.route("/", web::get().to(general::root_handler));
    config.route("/status", web::get().to(general::health_check_handler));
    config.route("/list", web::get().to(general::collection));
    config.route("/list/{val}", web::get().to(general::collection));
    config.route("/countries", web::get().to(general::countries_handler));
    config.route("/something", web::get().to(something::perform_something));
}
