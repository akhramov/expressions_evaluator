#![feature(try_find, decl_macro)]
extern crate anyhow;
extern crate nom;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod expression;
mod http_server;
mod solver;

fn main() {
    http_server::server().launch();
}
