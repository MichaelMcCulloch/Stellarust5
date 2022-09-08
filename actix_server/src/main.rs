use actix_cors::Cors;
use actix_web::{get, middleware, web::Data, App, HttpResponse, HttpServer, Responder};

use listenfd::ListenFd;

#[get("/")]
pub async fn index(s: Data<&str>) -> impl Responder {
    HttpResponse::Ok().body(String::from(*s.get_ref()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .app_data(Data::new(""))
            .service(index)
    });

    server = if let Some(listener) = ListenFd::from_env().take_tcp_listener(0)? {
        log::info!("{:?}", listener);
        server.listen(listener)?
    } else {
        log::info!("starting on 0.0.0.0:8000");
        server.bind("0.0.0.0:8000")?
    };

    server.run().await
}
