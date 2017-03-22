#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate error_chain;
extern crate rocket;
extern crate reqwest;
extern crate rustsec;

use errors::ResultExt;

use std::io::Read;

use rustsec::{AdvisoryDatabase, Lockfile};
use rustsec::advisory::Advisory;
use rustsec::error::Error as RustSecError;
use rustsec::lockfile::Package;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

fn get_advisories_from_lockfile(owner: &str, name: &str) -> errors::Result<()> {
    let manifest_url = format!("https://raw.githubusercontent.com/{}/{}/master/Cargo.lock",
                            owner,
                            name);

    let mut resp = reqwest::get(manifest_url.as_str())
        .chain_err(|| "Unable to download Cargo.lock")?;

    let mut buffer = String::new();
    match resp.status() {
        &reqwest::StatusCode::Ok => {
            resp.read_to_string(&mut buffer)
                .chain_err(|| "Unable to read Cargo.lock body")?;
            let lockfile = Lockfile::from_toml(buffer.as_str())
                .chain_err(|| "Unable verify status");
                Ok(())
        },
        _ => {
            bail!("Bad status code retreiving Cargo.lock");
        }
    }
}

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}