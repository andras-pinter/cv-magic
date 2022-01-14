#[macro_use]
extern crate rocket;

mod cli;
mod error;
mod parser;
mod renderer;

use parser::Config;
use renderer::prelude::*;
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
    let mut tmpf = std::fs::File::create(tmpd.path().join(CV_TEX)).map_err(error::Error::from)?;
    let rendering = renderer
        .render(config)?
        .replace("{ ", "{")
        .replace(" }", "}");

    tmpf.write_all(rendering.as_bytes())
        .map_err(error::Error::from)?;
    std::fs::copy(
        renderer.current_dir().join("templates").join(CV_CLASS),
        tmpd.path().join(CV_CLASS),
    )
    .map_err(error::Error::from)?;
    if let Some(path) = config.globals.get(PROFILE_PIC) {
        let path = renderer
            .current_dir()
            .join(path.as_str().unwrap_or_default());
        let filename = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map(String::from)
            .unwrap_or_default();
        std::fs::copy(path, tmpd.path().join(filename)).map_err(error::Error::from)?;
    }

    let mut command = std::process::Command::new("latexmk");
    let child = command
        .current_dir(tmpd.path())
        .arg(CV_TEX)
        .arg("-pdf")
        .arg("-f")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .arg("-interaction=nonstopmode")
        .spawn()
        .map_err(error::Error::from)?;
    let output = child.wait_with_output().map_err(error::Error::from)?;

    match output.status.code() {
        Some(0) => rocket::fs::NamedFile::open(tmpd.path().join("cv.pdf"))
            .await
            .map_err(|e| error::Error::from(e).into()),
        _ => {
            eprintln!("Generation error occurred");
            eprintln!("------- stdout log --------");
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("------- stderr log --------");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            eprintln!("-------- end logs ---------");
            Err(error::Error::InternalRendererError.into())
        }
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
