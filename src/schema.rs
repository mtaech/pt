// @generated automatically by Diesel CLI.

diesel::table! {
    source_data (id) {
        id -> Nullable<Integer>,
        name -> Text,
        path -> Text,
        ext -> Text,
        size -> Nullable<Integer>,
        camera_model -> Nullable<Text>,
    }
}

diesel::table! {
    target_data (id) {
        id -> Nullable<Integer>,
        name -> Text,
        path -> Text,
        ext -> Text,
        size -> Nullable<Integer>,
        camera_model -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    source_data,
    target_data,
);
