#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

extern crate askama;

mod io;
mod params;

use io::{generate_id, get_paste, store_paste};
use params::HostHeader;

use askama::Template;

use rocket::http::{ContentType, Status};
use rocket::request::Form;
use rocket::response::content::{Content, Html};
use rocket::response::Redirect;
use rocket::Data;

use std::io::Read;

///
/// Homepage
///

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[get("/")]
fn index() -> Result<Html<String>, Status> {
    Index
        .render()
        .map(Html)
        .map_err(|_| Status::InternalServerError)
}

///
/// Submit Paste
///

#[derive(FromForm)]
struct IndexForm {
    val: String,
}

#[post("/", data = "<input>")]
fn submit(input: Form<IndexForm>) -> Redirect {
    let id = generate_id();
    let uri = uri!(show_paste: &id);
    store_paste(id, input.into_inner().val);
    Redirect::to(uri)
}

#[put("/", data = "<input>")]
fn submit_raw(input: Data, host: HostHeader) -> std::io::Result<String> {
    let mut data = String::new();
    input.open().take(1024 * 1000).read_to_string(&mut data)?;

    let id = generate_id();
    let uri = uri!(show_paste: &id);

    store_paste(id, data);

    match *host {
        Some(host) => Ok(format!("https://{}{}", host, uri)),
        None => Ok(format!("{}", uri)),
    }
}

///
/// Show paste page
///

#[get("/<key>")]
fn show_paste(key: String) -> Result<Content<String>, Status> {
    let mut splitter = key.splitn(2, '.');
    let key = splitter.next().ok_or_else(|| Status::NotFound)?;
    let _ext = splitter.next();

    // get() returns a read-only lock, we're not going to be writing to this key
    // again so we can hold this for as long as we want
    let entry = &*get_paste(key).ok_or_else(|| Status::NotFound)?;

    Ok(Content(ContentType::Plain, entry.to_string()))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, submit, submit_raw, show_paste])
        .launch();
}
