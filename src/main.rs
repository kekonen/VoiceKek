extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

fn main() {
    let mut core = Core::new().unwrap();

    println!("HELLO");

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    println!("1");

    let api = Api::configure(token).build(core.handle()).unwrap();
    println!("2");


    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        println!("3");
        // If the received update contains a new message...

        match update.kind {
            UpdateKind::Message(message) => {
                println!("Incoming message");
                if let MessageKind::Text {ref data, ..} = message.kind {
                    println!("5");
                    // Print received text message to stdout.
                    println!("<{}>: {}", &message.from.first_name, data);

                    // Answer message with "Hi".
                    api.spawn(message.text_reply(
                        format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                    ));
                }
            },
            UpdateKind::InlineQuery(inline_query) => {
                // InlineQuery { id: InlineQueryId("936883958196477061"),
                // from: User { id: UserId(218135295), first_name: "Daniel", last_name: Some("Naumetc"), username: Some("kekonen"), is_bot: false, language_code: Some("en") },
                // location: None, query: "", offset: "" }
                // type TRequests = telegram_bot::types::requests

                println!("id:{:?}", inline_query);
                let kek = telegram_bot::types::InlineQueryResultVoice::new("kek", "Title", "https://file-examples.com/wp-content/uploads/2017/11/file_example_OOG_1MG.ogg");

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
