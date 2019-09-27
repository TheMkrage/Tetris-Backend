#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use std::io;

fn rocket() -> rocket::Rocket {
    // Mount "/" route to serve contents of /static
    rocket::ignite().mount("/", StaticFiles::from("static"))
}

fn main() {
    rocket().launch();
}
