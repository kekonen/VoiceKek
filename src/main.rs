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
        if let UpdateKind::Message(message) = update.kind {
            println!("4");
            if let MessageKind::Text {ref data, ..} = message.kind {
                println!("5");
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                api.spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                ));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
