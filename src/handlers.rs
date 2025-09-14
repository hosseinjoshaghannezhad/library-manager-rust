use crate::db::establish_connection;
use crate::models::Author;
use crate::models::AuthorForm;
use crate::models::Book;
use crate::models::BookForm;
use crate::models::LibraryItem;
use crate::models::Publisher;
use crate::models::PublisherForm;
use crate::models::User;
use crate::models::UserForm;
use actix_web::{web, HttpResponse};


pub async fn add_book(book_data: web::Json<BookForm>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let book = Book::new(
        book_data.title.clone(),
        book_data.author_id,
        book_data.publisher_id,
        book_data.isbn.clone(),
        book_data.year,
        book_data.price,
        book_data.quantity,
    );
    book.add(&mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Book added successfully!"))
}
pub async fn update_book(
    path: web::Path<i32>,
    book_data: web::Json<BookForm>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let book = Book::new(
        book_data.title.clone(),
        book_data.author_id,
        book_data.publisher_id,
        book_data.isbn.clone(),
        book_data.year,
        book_data.price,
        book_data.quantity,
    );
    book.update(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Book updated successfully!"))
}
pub async fn delete_book(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    Book::delete(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Book deleted successfully!"))
}
pub async fn delete_forever_book(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    Book::delete_forever(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Book deleted forever successfully"))
}
pub async fn restore_book(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    Book::restore(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Book restored successfully!"))
}
pub async fn show_one_book(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let book = Book::show_one(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(book))
}
pub async fn show_all_books() -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let books =
        Book::show_all(&mut connection).map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(books))
}
pub async fn show_deleted_books() -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let books =
        Book::show_deleted(&mut connection).map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(books))
}

pub async fn add_user(user_data: web::Json<UserForm>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let user = User::new(
        user_data.name.clone(),
        user_data.membership_id.clone(),
        user_data.phone.clone(),
    );
    user.add(&mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("User added successfully!"))
}
pub async fn update_user(
    path: web::Path<i32>,
    user_data: web::Json<UserForm>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let user = User::new(
        user_data.name.clone(),
        user_data.membership_id.clone(),
        user_data.phone.clone(),
    );
    user.update(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("User updated successfully!"))
}
pub async fn delete_user(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    User::delete(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("User deleted successfully!"))
}
pub async fn delete_forever_user(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    User::delete_forever(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("User deleted forever successfully"))
}
pub async fn restore_user(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    User::restore(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("User restored successfully!"))
}
pub async fn show_one_user(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let user = User::show_one(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(user))
}
pub async fn show_all_users() -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let users =
        User::show_all(&mut connection).map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(users))
}
pub async fn show_deleted_users() -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let users =
        User::show_deleted(&mut connection).map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_author(
    author_data: web::Json<AuthorForm>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let author = Author::new(author_data.name.clone());
    author
        .add(&mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Author added successfully!"))
}
pub async fn update_author(
    path: web::Path<i32>,
    author_data: web::Json<AuthorForm>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let author = Author::new(author_data.name.clone());
    author
        .update(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Author updated successfully!"))
}
pub async fn delete_author(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    Author::delete(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Author deleted successfully!"))
}
pub async fn delete_forever_author(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    Author::delete_forever(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Author deleted forever successfully"))
}
pub async fn restore_author(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    Author::restore(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Author restored successfully!"))
}
pub async fn show_one_author(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let author = Author::show_one(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(author))
}
pub async fn show_all_authors() -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let authors =
        Author::show_all(&mut connection).map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(authors))
}
pub async fn show_deleted_authors() -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let authors = Author::show_deleted(&mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(authors))
}

pub async fn add_publisher(
    publisher_data: web::Json<PublisherForm>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let publisher = Publisher::new(publisher_data.name.clone());
    publisher
        .add(&mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Publisher added successfully!"))
}
pub async fn update_publisher(
    path: web::Path<i32>,
    publisher_data: web::Json<PublisherForm>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let publisher = Publisher::new(publisher_data.name.clone());
    publisher
        .update(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Publisher updated successfully!"))
}
pub async fn delete_publisher(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    Publisher::delete(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Publisher deleted successfully!"))
}
pub async fn delete_forever_publisher(
    path: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    Publisher::delete_forever(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Publisher deleted forever successfully"))
}
pub async fn restore_publisher(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    Publisher::restore(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Publisher restored successfully!"))
}
pub async fn show_one_publisher(path: web::Path<i32>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let publisher = Publisher::show_one(path.into_inner(), &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(publisher))
}
pub async fn show_all_publishers() -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let publisher =
        Publisher::show_all(&mut connection).map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(publisher))
}
pub async fn show_deleted_publishers() -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let publisher = Publisher::show_deleted(&mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(publisher))
}

//borrowing
pub async fn borrow(path: web::Path<(i32, i32)>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let (book_id, user_id) = path.into_inner();
    Book::borrow(book_id, user_id, &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    User::borrow(book_id, user_id, &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Book borrowed successfully"))
}
pub async fn giving_back(path: web::Path<(i32, i32)>) -> Result<HttpResponse, actix_web::Error> {
    let mut connection = establish_connection();
    let (book_id, user_id) = path.into_inner();
    Book::give_back(book_id, &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    User::give_back(book_id, user_id, &mut connection)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("Book gives back successfully"))
}
