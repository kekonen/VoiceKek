extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
#[macro_use]
extern crate diesel;



use diesel::prelude::*;



pub mod models;
pub mod schema;

use self::models::{Voice, NewVoice, Task};
// use self::schema::voices::dsl::*;

mod lib;
use lib::{establish_connection};
use self::lib::*;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;


fn main() {
    let connection = establish_connection();
    // let results = voices
    //     .load::<Voice>(&connection)
    //     .expect("Error loading posts");
    //     // .filter(voices::published.eq(true))

    // println!("Displaying {} posts", results.len());
    // for voice in results {
    //     println!("{:?}", voice);
    //     println!("-----------\n");
    //     // println!("{}", voice.body);
    // }

    let mut core = Core::new().unwrap();

    println!("HELLO");

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    println!("1. Got token");

    let api = Api::configure(token).build(core.handle()).unwrap();
    println!("2. Configured Telegram Api");


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
                        println!("Got Text <{}>: {}\nOwner: {:?}", &message.from.first_name, data, &message.from);

                        // Answer message with "Hi".
                        api.spawn(message.text_reply(
                            format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                        ));
                    },
                    MessageKind::Audio {ref data, ..} => {
                        println!("Got Audio <{}>: {:?}", &message.from.first_name, data);

                        api.spawn(message.text_reply(
                            format!("Hi, {}! You just sent audio", &message.from.first_name)
                        ));
                    },
                    MessageKind::Voice {ref data, ..} => {
                        println!("Got Voice <{}>: {:?}", &message.from.first_name, data);
                        // { file_id: "AwADBAADIQUAAu6jqFMgkXH89n2udwI", duration: 2, mime_type: Some("audio/ogg"), file_size: Some(3986) }
                        match data.file_size {
                            Some(value) => create_voice(&connection, &data.file_id, &(i64::from(message.from.id) as i32), &(data.duration as i32), &(value as i32)),
                            _ => create_voice(&connection, &data.file_id, &123, &(data.duration as i32), &0)
                        };
                        api.spawn(message.text_reply(
                            format!("Hi, {}! You just sent voice", &message.from.first_name)
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

                println!("id:{:?}", inline_query);
                let kek = telegram_bot::types::InlineQueryResultVoice::new("kek", "Title", "http://kekonen.club/static/hitman.ogg");

                let mut results = Vec::new();
                results.push(
                    telegram_bot::types::InlineQueryResult::InlineQueryResultVoice(
                        kek
                    )
                );

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

    let new_post = NewVoice {
        file_id: file_id,
        owner_id: owner_id,
        duration: duration,
        size: size,
    };

    diesel::insert_into(voices::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}