#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket_contrib;
extern crate ws;

mod piece_state;

use crate::piece_state::PieceState;

use std::cell::Cell;
use std::rc::Rc;

use rocket_contrib::serve::StaticFiles;
use std::io;
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};

use serde::{Deserialize, Serialize};

struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        return Ok(self.count.set(self.count.get() + 1));
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // parse the msg as text
        if let Ok(text) = msg.into_text() {
            // try to parse the message as json,
            // if valid json, echo the json to everyone connected
            // else send blank response OK(()) (send nothing)
            match serde_json::from_str::<PieceState>(&text) {
                Ok(obj) => {
                    println!("Received status:\n{:?}\n", obj);
                    return self.out.send(text);
                }
                Err(e) => {
                    println!("Could not parse status: {}\n", e);
                    return Ok(());
                },
            }
        }
        // default to blank result
        return Ok(());
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn rocket() -> rocket::Rocket {
    // Mount "/" route to serve contents of /static
    rocket::ignite().mount("/", StaticFiles::from("static"))
}

fn main() {
    let count = Rc::new(Cell::new(0));
    listen("127.0.0.1:3012", |out| Server {
        out: out,
        count: count.clone(),
    })
    .unwrap();
    //rocket().launch();
}
