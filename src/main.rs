#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket_contrib;
extern crate ws;

use std::cell::Cell;
use std::rc::Rc;

use rocket_contrib::serve::StaticFiles;
use std::io;
use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};

struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("open");
        return Ok(self.count.set(self.count.get() + 1));
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("message");
        //print!("{}", msg.into_text().unwrap());
        // Echo message back
        return self.out.broadcast(msg);
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
