mod cli;
mod error;
mod index;
mod parser;
mod renderer;

use error::Result;
use parser::Config;
use std::sync::Arc;
use warp::{http::StatusCode, Filter};

fn create_index(config_file: std::path::PathBuf) -> Result<Arc<index::Index>> {
    Config::new(config_file)
        .and_then(index::Index::new)
        .map(Arc::new)
}

#[paw::main]
#[tokio::main]
async fn main(args: cli::Cli) {
    match args.addr() {
        Ok(addr) => match create_index(args.config_file) {
            Ok(index) => {
                let current_dir = std::env::current_dir()
                    .unwrap_or_default();
                let routes = warp::get()
                    .and(warp::path::end())
                    .map(move || {
                        index
                            .render()
                            .map(warp::reply::html)
                            .map(|rendered| warp::reply::with_status(rendered, StatusCode::OK))
                            .unwrap_or_else(|_| {
                                warp::reply::with_status(
                                    warp::reply::html("Internal Server Error".to_string()),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                )
                            })
                    })
                    .or(warp::path("download").map(|| {
                        warp::reply::with_status("Not Implemented", StatusCode::NOT_IMPLEMENTED)
                    }))
                    .or(warp::path("assets").and(
                        warp::fs::dir(current_dir.join("assets"))
                    ));

                println!("Serving on: http://{}", addr);
                warp::serve(routes).run(addr).await;
            }
            Err(err) => eprintln!("{}", err),
        },
        Err(err) => eprintln!("{}", err),
    }
}
