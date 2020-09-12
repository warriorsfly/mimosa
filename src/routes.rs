use crate::handlers::{article, auth, tag, user};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("signup", web::post().to(auth::signup))
                    .route("login", web::post().to(auth::login)),
            )
            .service(
                web::scope("/users").route("/{id}", web::get().to(user::get_user)), // .route("/{id}", web::put().to(update_user))
                                                                                    // .route("/{id}", web::delete().to(delete_user))
                                                                                    // .route("", web::get().to(get_users))
            )
            .service(
                web::scope("/articles")
                    .route("", web::post().to(article::create_article))
                    .route("", web::get().to(article::search_articles))
                    .route("/tags", web::get().to(tag::get_tags))
                    .route("/{slug}", web::get().to(article::get_one_article))
                    .route("/{slug}", web::post().to(article::update_article))
                    .route("/{slug}", web::get().to(article::delete_article))
                    .route(
                        "/{slug}/favorite",
                        web::post().to(article::favorite_article),
                    )
                    .route(
                        "/{slug}/favorite",
                        web::delete().to(article::unfavorite_article),
                    )
                    .route("/feed", web::post().to(article::feed_articles)),
                // .route("/{id}", web::delete().to(delete_user))
                // .route("", web::get().to(get_users))
            ),
    );
}
