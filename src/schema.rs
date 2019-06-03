table! {
    tasks (id) {
        id -> Int4,
        chat_id -> Int4,
        message_type -> Int4,
        task -> Varchar,
        content -> Varchar,
        fullfilled -> Bool,
    }
}

table! {
    voices (id) {
        id -> Int4,
        file_id -> Varchar,
        hash_sha1 -> Nullable<Varchar>,
        owner_id -> Int4,
        title -> Nullable<Varchar>,
        duration -> Nullable<Int4>,
        size -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
    voices,
);
