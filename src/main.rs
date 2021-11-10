use std::fs::File;
use std::io::BufReader;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
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
    let city_name = &name.replace("+", " ");
    match LANDKREISE.get().unwrap().get(&city_name.to_ascii_lowercase()) {
        Some(city_long_name) => {
            context.insert("city_long_name", &city_long_name);
            context.insert("city_name", &city_name);
            Template::render("result", &context)
        }
        None => {
            search_form()
        }
    }
}

static LANDKREISE: OnceCell<HashMap<String, String>> = OnceCell::new();


#[derive(Deserialize)]
struct Entry {
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="Short name")]
    long_name: String,
}

#[tokio::main]
async fn main() {
    let landkreise_file = BufReader::new(File::open("static/landkreise.json").unwrap());
    LANDKREISE
        .set(
            serde_json::from_reader::<_, Vec<Entry>>(landkreise_file)
                .unwrap()
                .into_iter()
                .map(|entry| (entry.name.to_ascii_lowercase(), entry.long_name)).collect())
        .unwrap();

    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![search_form, show_inzidenz])
        .launch()
        .await
        .unwrap();
}
