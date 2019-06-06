use chrono::NaiveDateTime;

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

#[derive(Queryable, Debug)]
pub struct VoicePermission {
    pub id: i32,
    pub owner_chat_id: i32,
    pub voice_file_id: String,
    pub created_at: NaiveDateTime,
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

use super::schema::tasks;

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask<'a> {
    pub chat_id: &'a i32,
    pub message_type: &'a i32,
    pub task: &'a str,
    pub content: &'a str,
}

use super::schema::voice_permissions;

#[derive(Insertable)]
#[table_name="voice_permissions"]
pub struct NewVoicePermission<'a> {
    pub owner_chat_id: &'a i32,
    pub voice_file_id: &'a str,
}