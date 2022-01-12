#[macro_use]
extern crate rocket;

mod cli;
mod error;
mod parser;

use error::Result;
use parser::Config;
use rocket::fs::{relative, FileServer};
use rocket::State;
use rocket_dyn_templates::Template;

#[get("/")]
fn render(config: &State<Config>) -> Template {
    Template::render("index", config.inner())
}

#[paw::main]
#[rocket::main]
async fn main(args: cli::Cli) {
    match Config::new(args.config_file) {
        Ok(config) => {
            rocket::build()
                .manage(config)
                .attach(Template::fairing())
                .mount("/", routes![render])
                .mount("/assets", FileServer::from(relative!("assets")))
                .launch()
                .await
                .expect("Failed to start server");
        }
        Err(err) => eprintln!("{}", err),
    }
}
