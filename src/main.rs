#[macro_use]
extern crate rocket;

mod cli;
mod error;
mod parser;
mod renderer;

use parser::Config;
use rocket::fs::FileServer;
use rocket::response::status::Custom;
use rocket::State;
use rocket_dyn_templates::Template;
use std::io::Write;

#[get("/")]
fn render(config: &State<Config>) -> Template {
    Template::render("index", config.inner())
}

#[get("/cv")]
async fn download(
    config: &State<Config>,
    renderer: &State<renderer::Render>,
) -> Result<rocket::fs::NamedFile, Custom<String>> {
    let tmpd = tempfile::TempDir::new().map_err(error::Error::from)?;
    let mut tmpf = std::fs::File::create(tmpd.path().join("cv.tex")).map_err(error::Error::from)?;
    tmpf.write_all(renderer.render(config)?.as_bytes())
        .map_err(error::Error::from)?;
    std::fs::copy(
        renderer.current_dir().join("templates").join("cv.cls"),
        tmpd.path().join("cv.cls"),
    )
    .map_err(error::Error::from)?;
    fs_extra::dir::copy(
        renderer.current_dir().join("assets"),
        tmpd.path(),
        &fs_extra::dir::CopyOptions::new(),
    )
    .map_err(error::Error::from)?;
    let status = std::process::Command::new("latexmk")
        .current_dir(tmpd.path())
        .arg("cv.tex")
        .arg("-pdf")
        .arg("-f")
        .arg("-quiet")
        .arg("-interaction=nonstopmode")
        .status()
        .map_err(error::Error::from)?;

    match status.code() {
        Some(0) => rocket::fs::NamedFile::open(tmpd.path().join("cv.pdf"))
            .await
            .map_err(|e| error::Error::from(e).into()),
        _ => Err(error::Error::InternalRendererError.into()),
    }
}

#[paw::main]
#[rocket::main]
async fn main(args: cli::Cli) {
    let current_dir = std::env::current_dir().unwrap_or_default();

    match Config::new(args.config_file) {
        Ok(config) => match renderer::Render::new(&current_dir) {
            Ok(renderer) => rocket::build()
                .manage(config)
                .manage(renderer)
                .attach(Template::fairing())
                .mount("/", routes![render, download])
                .mount("/assets", FileServer::from(current_dir.join("assets")))
                .launch()
                .await
                .expect("Failed to start server"),
            Err(err) => eprintln!("{}", err),
        },
        Err(err) => eprintln!("{}", err),
    }
}
