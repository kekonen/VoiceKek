table! {
    voices (id) {
        id -> Int4,
        file_id -> Varchar,
        hash_sha1 -> Varchar,
        owner_id -> Varchar,
        title -> Varchar,
        duration -> Nullable<Int4>,
        size -> Nullable<Int4>,
    }
}
