use actix_web::{middleware, web, App, HttpServer};
use std::{io, sync::Mutex};
mod appstate;
mod config;
mod jsonerrors;
mod errors;
mod handlers;
mod helpers;
mod https;
mod content_types;
mod render;
mod views;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Enable all logging
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config = config::init();
    let https = config.https;
    let appstate = appstate::init(config.clone()).expect("Failed to build appstate. Your database might be corrupt. The migrations might have failed. Go back to an older version and create an export.");

    let server = HttpServer::new(move || {
        let data = web::Data::new(Mutex::new(appstate.clone()));
        let app = App::new()
            .app_data(data)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(actix_files::Files::new("/static", "static/").show_files_listing())
            .service(actix_files::Files::new("/.well-known", "static/well-known/").show_files_listing())
            .service(web::scope("/tpf").service(web::resource("").route(web::get().to(handlers::tpf::tpf))))
            .service(web::scope("/path").service(web::resource("").route(web::get().to(handlers::path::path))))
            .service(web::scope("/commit").service(web::resource("").route(web::post().to(handlers::commit::post_commit))))
            .service(web::scope("/validate").service(web::resource("").route(web::get().to(handlers::validate::validate))))
            .service(web::scope("/{path:[^{}]+}").service(web::resource("").route(web::get().to(handlers::resource::get_resource))))
            .service(web::scope("/").service(web::resource("").route(web::get().to(handlers::home::home))))
            .app_data(
                web::JsonConfig::default()
                    // register error_handler for JSON extractors.
                    .error_handler(jsonerrors::json_error_handler),
            )
            ;
        if https {
            // Needs upate
            // app.wrap(actix_web_middleware_redirect_https::RedirectHTTPS::default())
        }
        app
    });

    if config.https {
        // If there is no certificate file, or the certs are too old, start HTTPS initialization
        if std::fs::File::open(&config.cert_path).is_err() || crate::https::check_expiration_certs() {
            https::cert_init_server(&config).await.unwrap();
        }
        let https_config = crate::https::get_https_config(&config)
            .expect("HTTPS TLS Configuration with Let's Encrypt failed.");
        let endpoint = format!("{}:{}", config.ip, config.port_https);
        server.bind_rustls(&endpoint, https_config).expect(&*format!("Cannot bind to endpoint {}", &endpoint))
            .run()
            .await?;
            Ok(())
    } else {
        let endpoint = format!("{}:{}", config.ip, config.port);
        server
            .bind(&format!("{}:{}", config.ip, config.port)).expect(&*format!("Cannot bind to endpoint {}", &endpoint))
            .run()
            .await?;
        Ok(())
    }
}
