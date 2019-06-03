#[derive(Queryable, Debug)]
pub struct Voice {
    pub id: i32,
    pub file_id: String,
    pub hash_sha1: Option<String>,
    pub owner_id: i32,
    pub title: Option<String>,
    pub duration: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct Task {
    pub id: i32,
    pub chat_id: i32,
    pub message_type: i32,
    pub task: String,
    pub content: String,
    pub fullfilled: bool,
}

use super::schema::voices;

#[derive(Insertable)]
#[table_name="voices"]
pub struct NewVoice<'a> {
    pub file_id: &'a str,
    pub owner_id: &'a i32,
    pub duration: &'a i32,
    pub size: &'a i32,
}