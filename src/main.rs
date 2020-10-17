#![feature(proc_macro_hygiene, decl_macro)]

use std::fs::File;
use std::io::BufReader;
use rocket_contrib::{
    templates::Template,
    serve::StaticFiles,
};
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use serde::Deserialize;

#[macro_use] extern crate rocket;


#[get("/")]
fn search_form() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("search", &context)
}
#[get("/<name>")]
fn show_inzidenz(name: String) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("city_long_name", &LANDKREISE.get().unwrap()[&name.to_ascii_lowercase()]);
    context.insert("city_name", &name);
    Template::render("result", &context)
}

static LANDKREISE: OnceCell<HashMap<String, String>> = OnceCell::new();


#[derive(Deserialize)]
struct Entry {
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="Short name")]
    long_name: String,
}
fn main() {
    let landkreise_file = BufReader::new(File::open("static/landkreise.json").unwrap());
    LANDKREISE
        .set(serde_json::from_reader::<_, Vec<Entry>>(landkreise_file).unwrap().into_iter().map(|entry| (entry.name.to_ascii_lowercase(), entry.long_name)).collect())
        .unwrap();

    rocket::ignite()
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![search_form, show_inzidenz])
        .launch();
}
