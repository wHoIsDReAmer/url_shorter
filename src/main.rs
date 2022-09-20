#[macro_use]
extern crate rocket;

use std::net::IpAddr;
use rocket::Config;
use rocket::fs::FileServer;

#[derive(rocket::form::FromForm)]
struct Url {
    from: String,
    to: String
}

#[post("/set", data = "<url>")]
fn set<'a>(url: rocket::form::Form<Url>) -> &'a str {
    let db = sled::open("db").unwrap();
    db.insert(url.from.as_str(), url.to.as_str());
    "Success"
}

#[get("/")]
async fn short_url() -> Result<rocket::fs::NamedFile, std::io::Error> {
    rocket::fs::NamedFile::open("front/index.html").await
}

#[get("/<url>")]
fn redirect(url: String) -> rocket::response::Redirect {
    let db = sled::open("db").unwrap();
    if let Ok(value) = db.get(&url.as_str()) {
        if value.is_some() {
            return rocket::response::Redirect::to(String::from_utf8_lossy(value.unwrap().to_vec().as_slice()).to_string())
        }
        return rocket::response::Redirect::to("about:blank")
    }
    rocket::response::Redirect::to("about:blank")

}

#[launch]
fn rocket() -> _ {//std::env::current_exe().unwrap().to_str().unwrap()
    // Heroku Setting
    let config = Config {
        port: if let Ok(value) = std::env::var("PORT") { value.parse::<u16>().unwrap() } else { 3999 },
        ident: rocket::config::Ident::try_new("Server").unwrap(),
        address: "0.0.0.0".parse::<IpAddr>().unwrap(),
        log_level: rocket::log::LogLevel::Critical,
        ..Config::debug_default()
    };
    println!("Listen... {}", if let Ok(value) = std::env::var("PORT") { value.parse::<u16>().unwrap() } else { 3999 });

    rocket::custom(config)
        .mount("/", FileServer::from("front"))
        .mount("/", routes![set, short_url, redirect])
}