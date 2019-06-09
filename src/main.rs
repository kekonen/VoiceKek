extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
#[macro_use]
extern crate diesel;
use std::process::Command;




use diesel::prelude::*;



pub mod models;
pub mod schema;

use self::models::{Voice, NewVoice, Task, NewTask, VoicePermission, NewVoicePermission};
// use self::schema::voices::dsl::*;
// use self::schema::tasks::dsl::*;

mod lib;
use lib::{establish_connection};
use self::lib::*;

mod download_file;
use download_file::{download_file, get_hash};

use std::env;

use futures::Stream;
use futures::*;

use tokio_core::reactor::Core;
use telegram_bot::*;


fn main() {
    let connection = establish_connection();

    let mut core = Core::new().unwrap();

    println!("HELLO");

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    // let movable_token = Box::new(token);
    println!("1. Got token");

    let api = Api::configure(token.clone()).build(core.handle()).unwrap();
    println!("2. Configured Telegram Api");

    // let future = api.send(GetMe);
    // future.and_then(|me| Ok(println!("===<>{:?}", me)));

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        println!("3. Started polling");
        // If the received update contains a new message...

        match update.kind {
            UpdateKind::Message(message) => {
                println!("Incoming message");
                match message.kind {
                    MessageKind::Text {ref data, ..} => {
                        // Print received text message to stdout.
                        println!("-> Got Text <{}>: {}\n-> Owner: {:?}", &message.from.first_name, data, &message.from);
                        // use schema::tasks;
                        use self::schema::tasks::dsl::*;
                        use self::schema::voices::dsl::*;

                        let sender_chat_id = &(i64::from(message.from.id) as i32);

                        let found_tasks = tasks
                            .filter(chat_id.eq(sender_chat_id))
                            .filter(message_type.eq(&0))
                            .filter(fullfilled.ne(&true))
                            .limit(1)
                            .load::<Task>(&connection)
                            .expect("Error loading posts");

                        match found_tasks.len() {
                            0 => println!("Not Found"),
                            1 => {
                                
                                let found_task = &found_tasks[0];
                                match found_task.task.as_ref() {
                                    "saveTitle" => {
                                        match voices.filter(title.eq(&data)).first::<Voice>(&connection) {
                                            Ok(_) => {
                                                api.spawn(message.text_reply(
                                                    format!("Hi, {}! You just wrote '{}' and it already exists, write another one", &message.from.first_name, data)
                                                ));
                                            }, _ => {
                                                
                                                let downloaded_file = download_file(&token.clone(), &found_task.content, &format!("voices/{}.ogg", &found_task.content));

                                                match downloaded_file {
                                                    Some((filesize, hash)) => {
                                                        println!("Going to update \nchat_id:'{}',\nfileId:'{}', ", sender_chat_id, found_task.content);

                                                        let found_voice = voices
                                                            .filter(owner_id.eq(sender_chat_id))
                                                            .filter(file_id.eq(found_task.content.to_owned()))
                                                            .first::<Voice>(&connection)
                                                            .expect("Error loading posts");

                                                        let found_voices = voices
                                                            .filter(owner_id.eq(sender_chat_id))
                                                            .filter(file_id.eq(found_task.content.to_owned()));

                                                        let voice_updated = diesel::update(found_voices).set((
                                                            title.eq(data),
                                                            size.eq(filesize as i32),
                                                            hash_b2s.eq(hash),
                                                            active.eq(true),
                                                        )).execute(&connection).unwrap();
                                                        println!("Voice updated -> {:?}", voice_updated);

                                                        let task_updated = diesel::update(tasks
                                                            .filter(chat_id.eq(sender_chat_id))
                                                            .filter(message_type.eq(&0))
                                                            .filter(fullfilled.ne(&true))
                                                        ).set(fullfilled.eq(true)).execute(&connection).unwrap();
                                                        println!("Task updated -> {:?}", task_updated);

                                                        let permission_created = create_voice_permission(&connection, &found_voice.id, sender_chat_id, &found_task.content);
                                                        println!("found savetitle")
                                                    },
                                                    _ => println!("Couldn't download the file!")
                                                }
                                            }
                                        }
                                    },
                                    "saveTitle.mp3" => {
                                        match voices.filter(title.eq(&data)).first::<Voice>(&connection) {
                                            Ok(_) => {
                                                api.spawn(message.text_reply(
                                                    format!("Hi, {}! You just wrote '{}' and it already exists, write another one", &message.from.first_name, data)
                                                ));
                                            }, _ => {
                                                let mp3_filename = format!("mp3/{}.mp3", &found_task.content);
                                                let voices_filename = format!("voices/{}.ogg", &found_task.content);
                                                let downloaded_file = download_file(&token.clone(), &found_task.content, &mp3_filename);

                                                // let ls = Command::new("ls").args(&["mp3"]).output();
                                                // println!("=>{:?}", ls);

                                                //ffmpeg -i mp3/smth.mp3 -ac 1 -map 0:a -codec:a libopus -b:a 128k -vbr off -ar 24000 voices/testlib.ogg
                                                println!("Running for {}", mp3_filename);
                                                let covert_mp3_to_ogg = Command::new("ffmpeg")
                                                .args(&[
                                                    "-i",
                                                    &mp3_filename,
                                                    "-ac",
                                                    "1",
                                                    "-map",
                                                    "0:a",
                                                    "-codec:a",
                                                    "libopus",
                                                    "-b:a",
                                                    "128k",
                                                    // "vbr",
                                                    // "off",
                                                    // "ar",
                                                    // "24000",
                                                    &voices_filename,
                                                ])
                                                .output();
                                                
                                                println!("Result ==>{:?}", covert_mp3_to_ogg);

                                                let file = std::fs::File::open(&voices_filename)?;
                                                let filelen = file.metadata().unwrap().len();

                                                let voice_hash = get_hash(&voices_filename);

                                                println!("Going to update \nchat_id:'{}',\nfileId:'{}', ", sender_chat_id, found_task.content);

                                                let found_voice = voices
                                                    .filter(owner_id.eq(sender_chat_id))
                                                    .filter(file_id.eq(found_task.content.to_owned()))
                                                    .first::<Voice>(&connection)
                                                    .expect("Error loading posts");

                                                let found_voices = voices
                                                    .filter(owner_id.eq(sender_chat_id))
                                                    .filter(file_id.eq(found_task.content.to_owned()));

                                                let voice_updated = diesel::update(found_voices).set((
                                                    title.eq(data),
                                                    hash_b2s.eq(voice_hash),
                                                    active.eq(true),
                                                    size.eq(Some(filelen as i32)),
                                                )).execute(&connection).unwrap();
                                                println!("Voice updated -> {:?}", voice_updated);

                                                let task_updated = diesel::update(tasks
                                                    .filter(chat_id.eq(sender_chat_id))
                                                    .filter(message_type.eq(&0))
                                                    .filter(fullfilled.ne(&true))
                                                ).set(fullfilled.eq(true)).execute(&connection).unwrap();
                                                println!("Task updated -> {:?}", task_updated);

                                                let permission_created = create_voice_permission(&connection, &found_voice.id, sender_chat_id, &found_task.content);
                                                println!("found savetitle")
                                            }
                                        }
                                    },
                                    _ => println!("Found unknown message type"),
                                }
                            },
                            _ => println!("Found too much!!! This is not possible!"),
                        }

                        api.spawn(message.text_reply(
                            format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                        ));
                    },
                    MessageKind::Audio {ref data, ..} => {
                        println!("Got Audio <{}>: {:?}", &message.from.first_name, data);

                        let voice = match data.file_size {
                            Some(value) => create_voice(&connection, &data.file_id, &(i64::from(message.from.id) as i32), &(data.duration as i32), &(value as i32)),
                            _ => create_voice(&connection, &data.file_id, &123, &(data.duration as i32), &0)
                        };

                        create_task(&connection, &(i64::from(message.from.id) as i32), &0, "saveTitle.mp3", &voice.file_id);

                        api.spawn(message.text_reply(
                            format!("Hi, {}! You just sent audio", &message.from.first_name)
                        ));
                    },
                    MessageKind::Voice {ref data, ..} => {
                        use self::schema::voices::dsl::*;

                        println!("Got Voice <{:?}>: {:?}", &message.from, data);

                        let sender_chat_id = &(i64::from(message.from.id) as i32);

                        let found_voice = voices
                            // .filter(owner_chat_id.eq(sender_chat_id))
                            .filter(size.eq(&(data.file_size.unwrap() as i32)))
                            .first::<Voice>(&connection);

                        match found_voice {
                            Ok(the_voice) => {
                                match check_if_same_voice(&token.clone(), &data.file_id, &the_voice.hash_b2s.unwrap()) {
                                    Some(_) => {
                                        println!("Found creating rules!======");

                                        let created_permission = create_voice_permission(&connection, &the_voice.id, &sender_chat_id, &the_voice.file_id);
                                        println!("Added permission: {:?}", created_permission);
                                    },
                                    _ => {
                                        println!("similar size but no, not exist!======");

                                        let voice = match data.file_size {
                                            Some(value) => create_voice(&connection, &data.file_id, &(i64::from(message.from.id) as i32), &(data.duration as i32), &(value as i32)),
                                            _ => create_voice(&connection, &data.file_id, &123, &(data.duration as i32), &0)
                                        };

                                        create_task(&connection, &(i64::from(message.from.id) as i32), &0, "saveTitle", &voice.file_id);
                                    },
                                }
                            },
                            _ => {
                                println!("not even similar!======");

                                let voice = match data.file_size {
                                    Some(value) => create_voice(&connection, &data.file_id, &(i64::from(message.from.id) as i32), &(data.duration as i32), &(value as i32)),
                                    _ => create_voice(&connection, &data.file_id, &123, &(data.duration as i32), &0)
                                };

                                create_task(&connection, &(i64::from(message.from.id) as i32), &0, "saveTitle", &voice.file_id);
                            }
                        }

                        api.spawn(message.text_reply(
                            format!("Hi, {}! I've got your voice! Please send the title...", &message.from.first_name)
                        ));
                    },
                    _ => println!("Other kind"),
                }
                // if let MessageKind::Text {ref data, ..} = message.kind {
                    
                // }
            },
            UpdateKind::InlineQuery(inline_query) => {
                // InlineQuery { id: InlineQueryId("936883958196477061"),
                // from: User { id: UserId(218135295), first_name: "Daniel", last_name: Some("Naumetc"), username: Some("kekonen"), is_bot: false, language_code: Some("en") },
                // location: None, query: "", offset: "" }
                // type TRequests = telegram_bot::types::requests
                use self::schema::voice_permissions::dsl::*;
                use self::schema::voices::dsl::*;

                println!("id:{:?}", inline_query);
                let found_perms: Vec<(String, Option<String>)> = voice_permissions.inner_join(voices)
                            .filter(owner_chat_id.eq(&(i64::from(inline_query.from.id) as i32)))
                            .filter(active.eq(true))
                            .select((file_id, title))
                            .load(&connection)
                            .expect("Error loading posts");
                
                println!("Perms ===> {:?}", found_perms);

                let mut results = Vec::new();

                let mut i = 0;
                for (f_id, p_title) in found_perms{
                    i+=1;
                    println!("{}, {:?}", f_id, p_title);
                    match p_title {
                        Some(ttl) => {
                            if ttl.contains(&inline_query.query) {
                                results.push(
                                    telegram_bot::types::InlineQueryResult::InlineQueryResultVoice(
                                        telegram_bot::types::InlineQueryResultVoice::new(i.to_string(), ttl, format!("http://kekonen.club/{}.ogg", f_id))
                                    )
                                )
                            }
                            
                        },
                        _ => println!("Found file with no title")
                    }
                }

                

                api.spawn(telegram_bot::types::requests::AnswerInlineQuery::new(inline_query.id, results));
                // api.spawn(inline_query);
            },
            _ => println!("kek"),
        }



        // if let UpdateKind::InlineQuery(inline_query) = update.kind {
        //     println!("4");
        // }

        Ok(())
    });

    core.run(future).unwrap();
}






fn create_voice<'a>(conn: &PgConnection, file_id: &'a str, owner_id: &'a i32, duration: &'a i32, size: &'a i32) -> Voice {
    use schema::voices;

    let new_voice = NewVoice {
        file_id: file_id,
        owner_id: owner_id,
        duration: duration,
        size: size,
    };

    diesel::insert_into(voices::table)
        .values(&new_voice)
        .get_result(conn)
        .expect("Error saving new post")
}

fn create_task<'a>(conn: &PgConnection, chat_id: &'a i32, message_type: &'a i32, task: &'a str, content: &'a str) -> Task {
    use schema::tasks;

    let new_task = NewTask {
        chat_id: chat_id,
        message_type: message_type,
        task: task,
        content: content,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(conn)
        .expect("Error saving new post")
}

fn create_voice_permission<'a>(conn: &PgConnection, voice_id: &'a i32, owner_chat_id: &'a i32, voice_file_id: &'a str) -> VoicePermission {
    use schema::voice_permissions;

    let new_permission = NewVoicePermission {
        voice_id: voice_id,
        owner_chat_id: owner_chat_id,
        voice_file_id: voice_file_id,
    };

    diesel::insert_into(voice_permissions::table)
        .values(&new_permission)
        .get_result(conn)
        .expect("Error saving new post")
}

fn check_if_same_voice(token: &str, one: &str, hashh: &str) -> Option<()> {
    let downloaded_file = download_file(&token.clone(), &one, &format!("voices/{}.ogg", one));

    match downloaded_file {
        Some((filesize, hash)) => {
            println!("HAAAAAAAAASH==> {}", hash);
            if hash == hashh {
                return Some(())
            }
            return None
        },
        _ => return None
    }
}