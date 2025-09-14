// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Nullable<Int4>,
        name -> Text,
        books -> Array<Int4>,
        is_deleted -> Bool,
    }
}

diesel::table! {
    books (id) {
        id -> Nullable<Int4>,
        title -> Text,
        author_id -> Int4,
        publisher_id -> Int4,
        isbn -> Text,
        year -> Int4,
        price -> Float4,
        quantity -> Int4,
        borrowed_by -> Array<Int4>,
        is_deleted -> Bool,
    }
}

diesel::table! {
    publishers (id) {
        id -> Nullable<Int4>,
        name -> Text,
        books -> Array<Int4>,
        is_deleted -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Int4>,
        name -> Text,
        membership_id -> Text,
        phone -> Text,
        current_books -> Array<Int4>,
        borrowed_history -> Array<Int4>,
        is_deleted -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    publishers,
    users,
);
// // @generated automatically by Diesel CLI.

// diesel::table! {
//     authors (id) {
//         id -> Nullable<Int4>,
//         name -> Text,
//         books -> Array<Int4>,
//         is_deleted -> Bool,
//     }
// }

// diesel::table! {
//     books (id) {
//         id -> Nullable<Int4>,
//         title -> Text,
//         author_id -> Int4,
//         publisher_id -> Int4,
//         isbn -> Text,
//         year -> Int4,
//         price -> Float4,
//         quantity -> Int4,
//         borrowed_by -> Array<Int4>,
//         is_deleted -> Bool,
//     }
// }

// diesel::table! {
//     publishers (id) {
//         id -> Nullable<Int4>,
//         name -> Text,
//         books -> Array<Int4>,
//         is_deleted -> Bool,
//     }
// }

// diesel::table! {
//     users (id) {
//         id -> Nullable<Int4>,
//         membership_id -> Text,
//         phone -> Text,
//         current_books -> Array<Int4>,
//         borrowed_history -> Array<Int4>,
//         is_deleted -> Bool,
//     }
// }

// diesel::allow_tables_to_appear_in_same_query!(
//     authors,
//     books,
//     publishers,
//     users,
// );
