use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/users")
                    .route("/all", web::get().to(handlers::show_all_users))
                    .route("/deleted", web::get().to(handlers::show_deleted_users))
                    .route("/{id}", web::get().to(handlers::show_one_user))
                    .route("/create", web::post().to(handlers::add_user))
                    .route("/{id}", web::put().to(handlers::update_user))
                    .route("/{id}", web::delete().to(handlers::delete_user))
                    .route("/{id}/permanent", web::delete().to(handlers::delete_forever_user))
                    .route("/{id}/restore", web::post().to(handlers::restore_user))
            )
            .service(
                web::scope("/books")
                    .route("/all", web::get().to(handlers::show_all_books))
                    .route("/deleted", web::get().to(handlers::show_deleted_books))
                    .route("/{id}", web::get().to(handlers::show_one_book))
                    .route("/create", web::post().to(handlers::add_book))
                    .route("/{id}", web::put().to(handlers::update_book))
                    .route("/{id}", web::delete().to(handlers::delete_book))
                    .route("/{id}/permanent", web::delete().to(handlers::delete_forever_book))
                    .route("/{id}/restore", web::post().to(handlers::restore_book))
            )
            .service(
                web::scope("/borrowing")
                    .route("/borrow/{user_id}/{book_id}", web::get().to(handlers::borrow))
                    .route("/return/{user_id}/{book_id}", web::get().to(handlers::giving_back))
            )
            .service(
                web::scope("/authors")
                .route("/all", web::get().to(handlers::show_all_authors))
                .route("/deleted", web::get().to(handlers::show_deleted_authors))
                .route("/{id}", web::get().to(handlers::show_one_author))
                .route("/create", web::post().to(handlers::add_author))
                .route("/{id}", web::put().to(handlers::update_author))
                .route("/{id}", web::delete().to(handlers::delete_author))
                .route("/{id}/permanent", web::delete().to(handlers::delete_forever_author))
                .route("/{id}/restore", web::post().to(handlers::restore_author))
            )
            .service(
                web::scope("/publishers")
                .route("/all", web::get().to(handlers::show_all_publishers))
                .route("/deleted", web::get().to(handlers::show_deleted_publishers))
                .route("/{id}", web::get().to(handlers::show_one_publisher))
                .route("/create", web::post().to(handlers::add_publisher))
                .route("/{id}", web::put().to(handlers::update_publisher))
                .route("/{id}", web::delete().to(handlers::delete_publisher))
                .route("/{id}/permanent", web::delete().to(handlers::delete_forever_publisher))
                .route("/{id}/restore", web::post().to(handlers::restore_publisher))
            )
    );
}