use crate::schema::{authors, books, publishers, users};
use diesel::sql_types::Integer;
use diesel::prelude::*;
use std::vec::Vec;
pub trait LibraryItem<T> {
    fn add(&self, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized;
    fn update(
        &self,
        item_id: i32,
        connection: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error>
    where
        Self: Sized;
    fn delete(item_id: i32, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized;
    fn delete_forever(
        item_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error>
    where
        Self: Sized;
    fn restore(item_id: i32, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized;
    fn show_one(id: i32, connection: &mut PgConnection) -> Result<T, diesel::result::Error>
    where
        Self: Sized;
    fn show_all(connection: &mut PgConnection) -> Result<Vec<T>, diesel::result::Error>
    where
        Self: Sized;
    fn show_deleted(connection: &mut PgConnection) -> Result<Vec<T>, diesel::result::Error>
    where
        Self: Sized;
}

//Book
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = books)]
pub struct Book {
    id: Option<i32>,
    title: String,
    author_id: i32,
    publisher_id: i32,
    isbn: String,
    year: i32,
    price: f32,
    quantity: i32,
    borrowed_by: Vec<i32>,
    is_deleted: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct BookForm {
    pub title: String,
    pub author_id: i32,
    pub publisher_id: i32,
    pub isbn: String,
    pub year: i32,
    pub price: f32,
    pub quantity: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BookDTO {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
    pub publisher_id: i32,
    pub isbn: String,
    pub year: i32,
    pub price: f32,
    pub quantity: i32,
    pub borrowed_by: Vec<i32>,
    pub is_deleted: bool,
}
impl From<Book> for BookDTO {
    fn from(book: Book) -> Self {
        BookDTO {
            id: book.id.unwrap(),
            title: book.title,
            author_id: book.author_id,
            publisher_id: book.publisher_id,
            isbn: book.isbn,
            year: book.year,
            price: book.price,
            quantity: book.quantity,
            borrowed_by: book.borrowed_by,
            is_deleted: book.is_deleted,
        }
    }
}
impl Book {
    pub fn new(
        title_: String,
        author_id_: i32,
        publisher_id_: i32,
        isbn_: String,
        year_: i32,
        price_: f32,
        quantity_: i32,
    ) -> Book {
        Book {
            id: None,
            title: title_,
            author_id: author_id_,
            publisher_id: publisher_id_,
            isbn: isbn_,
            year: year_,
            price: price_,
            quantity: quantity_,
            borrowed_by: Vec::new(),
            is_deleted: false,
        }
    }
    pub fn cleanup_borrowed_books(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::books::dsl::*;
        let all_books: Vec<Book> = books.load(connection)?;
        for book in all_books {
            if book.borrowed_by.contains(&user_id) {
                let updated_borrowed_by: Vec<i32> = book
                    .borrowed_by
                    .into_iter()
                    .filter(|&borrowed_id| borrowed_id != user_id)
                    .collect();
                diesel::update(books.filter(id.eq(book.id.unwrap())))
                    .set(borrowed_by.eq(updated_borrowed_by))
                    .execute(connection)?;
            }
        }
        Ok(())
    }
    pub fn borrow(
        book_id: i32,
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::books::dsl::*;
        diesel::update(books.filter(id.eq(book_id)))
            .set(quantity.eq(quantity - 1))
            .execute(connection)?;
        diesel::update(books.filter(id.eq(book_id)))
            .set(
                borrowed_by.eq(diesel::dsl::sql("array_append(borrowed_by, ")
                    .bind::<Integer, _>(user_id)
                    .sql(")")),
            )
            .execute(connection)?;
        Ok(())
    }
    pub fn give_back(
        book_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::books::dsl::*;
        diesel::update(books.filter(id.eq(book_id)))
            .set(quantity.eq(quantity + 1))
            .execute(connection)?;
        Ok(())
    }
}
impl LibraryItem<BookDTO> for Book {
    fn add(&self, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        let new_book: Book = diesel::insert_into(books::table)
            .values(self)
            .get_result(connection)?;
        Author::add_book(self.author_id, new_book.id.unwrap(), connection)?;
        Publisher::add_book(self.publisher_id, new_book.id.unwrap(), connection)?;
        Ok(new_book)
    }
    fn update(
        &self,
        book_id: i32,
        connection: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::update(books::table.filter(books::id.eq(book_id)))
            .set((
                books::title.eq(self.title.clone()),
                books::author_id.eq(self.author_id),
                books::publisher_id.eq(self.publisher_id),
                books::isbn.eq(self.isbn.clone()),
                books::year.eq(self.year),
                books::price.eq(self.price),
                books::quantity.eq(self.quantity),
            ))
            .get_result(connection)
    }
    fn delete(book_id: i32, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        if User::is_book_carrying(book_id, connection)? {
            return Err(diesel::result::Error::QueryBuilderError(
                "Cannot delete book, this book is carrying by someone".into(),
            ));
        }
        diesel::update(books::table.filter(books::id.eq(book_id)))
            .set(books::is_deleted.eq(true))
            .get_result(connection)
    }
    fn delete_forever(
        book_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error>
    where
        Self: Sized,
    {
        User::cleanup_borrowed_history(book_id, connection)?;
        diesel::delete(books::table.filter(books::id.eq(book_id))).execute(connection)?;
        Ok(())
    }
    fn restore(book_id: i32, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::update(books::table.filter(books::id.eq(book_id)))
            .set(books::is_deleted.eq(false))
            .get_result(connection)
    }
    fn show_one(
        book_id: i32,
        connection: &mut PgConnection,
    ) -> Result<BookDTO, diesel::result::Error>
    where
        Self: Sized,
    {
        let book: Book = books::table
            .filter(books::id.eq(book_id))
            .first(connection)?;
        Ok(book.into())
    }
    fn show_all(connection: &mut PgConnection) -> Result<Vec<BookDTO>, diesel::result::Error>
    where
        Self: Sized,
    {
        let books: Vec<Book> = books::table
            .filter(books::is_deleted.eq(false))
            .filter(books::quantity.gt(0))
            .load::<Book>(connection)?;
        Ok(books.into_iter().map(BookDTO::from).collect())
    }
    fn show_deleted(connection: &mut PgConnection) -> Result<Vec<BookDTO>, diesel::result::Error>
    where
        Self: Sized,
    {
        let books: Vec<Book> = books::table
            .filter(books::is_deleted.eq(true))
            .load::<Book>(connection)?;
        Ok(books.into_iter().map(BookDTO::from).collect())
    }
}

//User
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    id: Option<i32>,
    name: String,
    membership_id: String,
    phone: String,
    current_books: Vec<i32>,
    borrowed_history: Vec<i32>,
    is_deleted: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserForm {
    pub name: String,
    pub membership_id: String,
    pub phone: String,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserDTO {
    pub name: String,
    pub id: i32,
    pub membership_id: String,
    pub phone: String,
    pub current_books: Vec<i32>,
    pub borrowed_history: Vec<i32>,
    pub is_deleted: bool,
}
impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            name: user.name.clone(),
            id: user.id.unwrap(),
            membership_id: user.membership_id,
            phone: user.phone,
            current_books: user.current_books,
            borrowed_history: user.borrowed_history,
            is_deleted: user.is_deleted,
        }
    }
}
impl User {
    pub fn new(name_: String, membership_id_: String, phone_: String) -> User {
        User {
            id: None,
            name: name_,
            membership_id: membership_id_,
            phone: phone_,
            current_books: Vec::new(),
            borrowed_history: Vec::new(),
            is_deleted: false,
        }
    }
    pub fn is_book_carrying(
        book_id: i32,
        connection: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error>
    where
        Self: Sized,
    {
        let all_users: Vec<User> = users::table.load(connection)?;
        for user in all_users {
            if user.current_books.contains(&book_id) {
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub fn cleanup_borrowed_history(
        book_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let all_users: Vec<User> = users.load(connection)?;
        for user in all_users {
            if user.borrowed_history.contains(&book_id) {
                let updated_borrowed_history: Vec<i32> = user
                    .borrowed_history
                    .into_iter()
                    .filter(|&book_id_| book_id_ != book_id)
                    .collect();
                diesel::update(users.filter(id.eq(user.id.unwrap())))
                    .set(borrowed_history.eq(updated_borrowed_history))
                    .execute(connection)?;
            }
        }
        Ok(())
    }
    pub fn borrow(
        book_id: i32,
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(
                users::current_books.eq(diesel::dsl::sql("array_append(current_books, ")
                    .bind::<Integer, _>(book_id)
                    .sql(")")),
            )
            .execute(connection)?;
        diesel::update(users::table.filter(users::id.eq(user_id)))
        .set(
            users::borrowed_history.eq(diesel::dsl::sql("array_append(borrowed_history, ")
                .bind::<Integer, _>(book_id)
                .sql(")")),
        )
        .execute(connection)?;
        Ok(())
    }
    pub fn give_back(
        book_id: i32,
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(
                users::current_books.eq(diesel::dsl::sql("array_remove(current_books, ")
                    .bind::<Integer, _>(book_id)
                    .sql(")")),
            )
            .execute(connection)?;
        Ok(())
    }
}
impl LibraryItem<UserDTO> for User {
    fn add(&self, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::insert_into(users::table)
            .values(self)
            .get_result(connection)
    }
    fn update(
        &self,
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set((
                users::membership_id.eq(self.membership_id.clone()),
                users::phone.eq(self.phone.clone()),
            ))
            .get_result(connection)
    }
    fn delete(user_id: i32, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        let book_list = users::table
            .filter(users::id.eq(user_id))
            .select(users::current_books)
            .first::<Vec<i32>>(connection)?;

        if !book_list.is_empty() {
            return Err(diesel::result::Error::QueryBuilderError(
                "Cannot delete user, this user has borrowed some books.".into(),
            ));
        }
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::is_deleted.eq(true))
            .get_result(connection)
    }
    fn delete_forever(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error>
    where
        Self: Sized,
    {
        Book::cleanup_borrowed_books(user_id, connection)?;
        diesel::delete(users::table.filter(users::id.eq(user_id))).execute(connection)?;
        Ok(())
    }
    fn restore(user_id: i32, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::is_deleted.eq(false))
            .get_result(connection)
    }
    fn show_one(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<UserDTO, diesel::result::Error>
    where
        Self: Sized,
    {
        let user: User = users::table
            .filter(users::id.eq(user_id))
            .first(connection)?;
        Ok(user.into())
    }
    fn show_all(connection: &mut PgConnection) -> Result<Vec<UserDTO>, diesel::result::Error>
    where
        Self: Sized,
    {
        let users: Vec<User> = users::table
            .filter(users::is_deleted.eq(false))
            .load::<User>(connection)?;
        Ok(users.into_iter().map(UserDTO::from).collect())
    }
    fn show_deleted(connection: &mut PgConnection) -> Result<Vec<UserDTO>, diesel::result::Error>
    where
        Self: Sized,
    {
        let users: Vec<User> = users::table
            .filter(users::is_deleted.eq(true))
            .load::<User>(connection)?;
        Ok(users.into_iter().map(UserDTO::from).collect())
    }
}

//Author
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = authors)]
pub struct Author {
    id: Option<i32>,
    name: String,
    books: Vec<i32>,
    is_deleted: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AuthorForm {
    pub name: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthorDTO {
    pub id: i32,
    pub name: String,
    pub books: Vec<i32>,
    pub is_deleted: bool,
}
impl From<Author> for AuthorDTO {
    fn from(aouthor: Author) -> Self {
        AuthorDTO {
            id: aouthor.id.unwrap(),
            name: aouthor.name,
            books: aouthor.books,
            is_deleted: aouthor.is_deleted,
        }
    }
}
impl Author {
    pub fn new(name_: String) -> Author {
        Author {
            id: None,
            name: name_,
            books: Vec::new(),
            is_deleted: false,
        }
    }
    pub fn add_book(
        author_id: i32,
        book_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::authors::dsl::*;

        diesel::update(authors.filter(id.eq(author_id)))
            .set(books.eq(diesel::dsl::sql(&format!(
                "array_append(books, {})",
                book_id
            ))))
            .execute(connection)?;
        Ok(())
    }
}

impl LibraryItem<AuthorDTO> for Author {
    fn add(&self, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::insert_into(authors::table)
            .values(self)
            .get_result(connection)
    }
    fn update(
        &self,
        author_id_: i32,
        connection: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::update(authors::table.filter(authors::id.eq(author_id_)))
            .set((authors::name.eq(self.name.clone()),))
            .get_result(connection)
    }
    fn delete(author_id_: i32, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        let book_list = authors::table
            .filter(authors::id.eq(author_id_))
            .select(authors::books)
            .first::<Vec<i32>>(connection)?;

        if !book_list.is_empty() {
            return Err(diesel::result::Error::QueryBuilderError(
                "Cannot delete author, some books are associated with this author".into(),
            ));
        }
        diesel::update(authors::table.filter(authors::id.eq(author_id_)))
            .set(authors::is_deleted.eq(true))
            .get_result(connection)
    }
    fn delete_forever(
        author_id_: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::delete(authors::table.filter(authors::id.eq(author_id_))).execute(connection)?;
        Ok(())
    }
    fn restore(
        author_id_: i32,
        connection: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::update(authors::table.filter(authors::id.eq(author_id_)))
            .set(authors::is_deleted.eq(false))
            .get_result(connection)
    }
    fn show_one(
        author_id_: i32,
        connection: &mut PgConnection,
    ) -> Result<AuthorDTO, diesel::result::Error>
    where
        Self: Sized,
    {
        let author: Author = authors::table
            .filter(authors::id.eq(author_id_))
            .first(connection)?;
        Ok(author.into())
    }
    fn show_all(connection: &mut PgConnection) -> Result<Vec<AuthorDTO>, diesel::result::Error>
    where
        Self: Sized,
    {
        let authors: Vec<Author> = authors::table
            .filter(authors::is_deleted.eq(false))
            .load::<Author>(connection)?;
        Ok(authors.into_iter().map(AuthorDTO::from).collect())
    }
    fn show_deleted(connection: &mut PgConnection) -> Result<Vec<AuthorDTO>, diesel::result::Error>
    where
        Self: Sized,
    {
        let authors: Vec<Author> = authors::table
            .filter(authors::is_deleted.eq(true))
            .load::<Author>(connection)?;
        Ok(authors.into_iter().map(AuthorDTO::from).collect())
    }
}

//Publisher
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = publishers)]
pub struct Publisher {
    id: Option<i32>,
    name: String,
    books: Vec<i32>,
    is_deleted: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PublisherForm {
    pub name: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PublisherDTO {
    pub id: i32,
    pub name: String,
    books: Vec<i32>,
    pub is_deleted: bool,
}
impl From<Publisher> for PublisherDTO {
    fn from(publisher: Publisher) -> Self {
        PublisherDTO {
            id: publisher.id.unwrap(),
            name: publisher.name,
            books: publisher.books,
            is_deleted: publisher.is_deleted,
        }
    }
}
impl Publisher {
    pub fn new(name_: String) -> Publisher {
        Publisher {
            id: None,
            name: name_,
            books: Vec::new(),
            is_deleted: false,
        }
    }
    pub fn add_book(
        publisher_id: i32,
        book_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::publishers::dsl::*;

        diesel::update(publishers.filter(id.eq(publisher_id)))
            .set(books.eq(diesel::dsl::sql(&format!(
                "array_append(books, {})",
                book_id
            ))))
            .execute(connection)?;
        Ok(())
    }
}
impl LibraryItem<PublisherDTO> for Publisher {
    fn add(&self, connection: &mut PgConnection) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::insert_into(publishers::table)
            .values(self)
            .get_result(connection)
    }
    fn update(
        &self,
        publisher_id_: i32,
        connection: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::update(publishers::table.filter(publishers::id.eq(publisher_id_)))
            .set((publishers::name.eq(self.name.clone()),))
            .get_result(connection)
    }
    fn delete(
        publisher_id_: i32,
        connection: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        let book_list = publishers::table
            .filter(publishers::id.eq(publisher_id_))
            .select(publishers::books)
            .first::<Vec<i32>>(connection)?;

        if !book_list.is_empty() {
            return Err(diesel::result::Error::QueryBuilderError(
                "Cannot delete publisher, some books are associated with this publisher".into(),
            ));
        }
        diesel::update(publishers::table.filter(publishers::id.eq(publisher_id_)))
            .set(publishers::is_deleted.eq(true))
            .get_result(connection)
    }
    fn delete_forever(
        publisher_id_: i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(publishers::table.filter(publishers::id.eq(publisher_id_)))
            .execute(connection)?;
        Ok(())
    }
    fn restore(
        publisher_id_: i32,
        connection: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error>
    where
        Self: Sized,
    {
        diesel::update(publishers::table.filter(publishers::id.eq(publisher_id_)))
            .set(publishers::is_deleted.eq(false))
            .get_result(connection)
    }
    fn show_one(
        publisher_id_: i32,
        connection: &mut PgConnection,
    ) -> Result<PublisherDTO, diesel::result::Error>
    where
        Self: Sized,
    {
        let publisher: Publisher = publishers::table
            .filter(publishers::id.eq(publisher_id_))
            .first(connection)?;
        Ok(publisher.into())
    }
    fn show_all(connection: &mut PgConnection) -> Result<Vec<PublisherDTO>, diesel::result::Error>
    where
        Self: Sized,
    {
        let publishers: Vec<Publisher> = publishers::table
            .filter(publishers::is_deleted.eq(false))
            .load::<Publisher>(connection)?;
        Ok(publishers.into_iter().map(PublisherDTO::from).collect())
    }
    fn show_deleted(
        connection: &mut PgConnection,
    ) -> Result<Vec<PublisherDTO>, diesel::result::Error>
    where
        Self: Sized,
    {
        let publishers: Vec<Publisher> = publishers::table
            .filter(publishers::is_deleted.eq(true))
            .load::<Publisher>(connection)?;
        Ok(publishers.into_iter().map(PublisherDTO::from).collect())
    }
}