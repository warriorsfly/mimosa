use crate::auth::get_identity_service;
use crate::awc::add_awc;
use crate::cache::add_cache;
use crate::config::CONFIG;
use crate::database::add_pool;
use crate::routes::routes;
use crate::state::new_state;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use listenfd::ListenFd;

pub async fn server() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let data = new_state::<String>();
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .configure(add_pool)
            .configure(add_cache)
            .configure(add_awc)
            .wrap(Cors::new().supports_credentials().finish())
            .wrap(Logger::default())
            .wrap(get_identity_service())
            .app_data(data.clone())
            .configure(routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };

    server.run().await
}