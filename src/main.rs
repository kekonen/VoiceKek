extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
#[macro_use]
extern crate diesel;


use self::models::{Voice};
use self::lib::*;
use diesel::prelude::*;

mod lib;
use lib::{establish_connection};

pub mod models;
pub mod schema;
use self::schema::voices::dsl::*;

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
