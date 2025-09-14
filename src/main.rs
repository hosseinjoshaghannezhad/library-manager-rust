use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};

mod db;
mod handlers;
mod models;
mod routes;
mod schema;

async fn serve_book_pages(req: HttpRequest) -> impl Responder {
    let path = req.path();
    let file_path = match path {
        "/book/show-all" => "static/book/ShowAll.html",
        "/book/show-deleted" => "static/book/ShowDeleted.html",
        "/book/new" => "static/book/New.html",
        "/book/details" => "static/book/Details.html",
        "/user/show-all" => "static/user/ShowAll.html",
        "/user/show-deleted" => "static/user/ShowDeleted.html",
        "/user/new" => "static/user/New.html",
        "/user/details" => "static/user/Details.html",
        "/author/show-all" => "static/author/ShowAll.html",
        "/author/show-deleted" => "static/author/ShowDeleted.html",
        "/author/new" => "static/author/New.html",
        "/author/details" => "static/author/Details.html",
        "/publisher/show-all" => "static/publisher/ShowAll.html",
        "/publisher/show-deleted" => "static/publisher/ShowDeleted.html",
        "/publisher/new" => "static/publisher/New.html",
        "/publisher/details" => "static/publisher/Details.html",
        _ => return HttpResponse::NotFound().finish(),
    };

    actix_files::NamedFile::open(file_path)
        .map(|file| file.into_response(&req))
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .configure(routes::config)
            .service(Files::new("/static", "./static"))
            .route("/book/show-all", web::get().to(serve_book_pages))
            .route("/book/show-deleted", web::get().to(serve_book_pages))
            .route("/book/new", web::get().to(serve_book_pages))
            .route("/book/details", web::get().to(serve_book_pages))
            .route("/user/show-all", web::get().to(serve_book_pages))
            .route("/user/show-deleted", web::get().to(serve_book_pages))
            .route("/user/new", web::get().to(serve_book_pages))
            .route("/user/details", web::get().to(serve_book_pages))
            .route("/author/show-all", web::get().to(serve_book_pages))
            .route("/author/show-deleted", web::get().to(serve_book_pages))
            .route("/author/new", web::get().to(serve_book_pages))
            .route("/author/details", web::get().to(serve_book_pages))
            .route("/publisher/show-all", web::get().to(serve_book_pages))
            .route("/publisher/show-deleted", web::get().to(serve_book_pages))
            .route("/publisher/new", web::get().to(serve_book_pages))
            .route("/publisher/details", web::get().to(serve_book_pages))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}