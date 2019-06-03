#[derive(Queryable, Debug)]
pub struct Voice {
    pub id: i32,
    pub file_id: String,
    pub hash_sha1: String,
    pub owner_id: String,
    pub title: String,
    pub duration: Option<i32>,
    pub size: Option<i32>,
}