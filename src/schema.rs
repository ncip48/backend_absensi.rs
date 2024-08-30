// @generated automatically by Diesel CLI.

diesel::table! {
    classrooms (classroom_id) {
        classroom_id -> Integer,
        #[max_length = 255]
        classroom_name -> Varchar,
        classroom_status -> Nullable<Bool>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 122]
        password -> Varchar,
        role -> Tinyint,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    classrooms,
    users,
);
