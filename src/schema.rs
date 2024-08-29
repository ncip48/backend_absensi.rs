// @generated automatically by Diesel CLI.

diesel::table! {
    classrooms (classroom_id) {
        classroom_id -> Integer,
        #[max_length = 255]
        classroom_name -> Varchar,
        classroom_status -> Nullable<Bool>,
    }
}
