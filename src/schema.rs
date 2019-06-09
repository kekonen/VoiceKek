table! {
    file_source (id) {
        id -> Int4,
        mime_type -> Varchar,
        hash_sha256 -> Varchar,
        voice_id -> Int4,
    }
}

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
    voice_permissions (id) {
        id -> Int4,
        voice_id -> Int4,
        owner_chat_id -> Int4,
        voice_file_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    voices (id) {
        id -> Int4,
        file_id -> Varchar,
        hash_sha256 -> Nullable<Varchar>,
        owner_id -> Int4,
        title -> Nullable<Varchar>,
        duration -> Nullable<Int4>,
        size -> Nullable<Int4>,
        active -> Bool,
    }
}

joinable!(file_source -> voices (voice_id));
joinable!(voice_permissions -> voices (voice_id));

allow_tables_to_appear_in_same_query!(
    file_source,
    tasks,
    voice_permissions,
    voices,
);
