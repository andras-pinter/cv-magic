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
                let routes = warp::get()
                    .and(warp::path::end())
                    .map(move || {
                        index
                            .render()
                            .map(|rendered| warp::reply::with_status(rendered, StatusCode::OK))
                            .unwrap_or_else(|_| {
                                warp::reply::with_status(
                                    "Internal Server Error".to_string(),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                )
                            })
                    })
                    .or(warp::path("download").map(|| {
                        warp::reply::with_status("Not Implemented", StatusCode::NOT_IMPLEMENTED)
                    }));

                println!("Serving on: http://{}", addr);
                warp::serve(routes).run(addr).await;
            }
            Err(err) => eprintln!("{}", err),
        },
        Err(err) => eprintln!("{}", err),
    }
}
