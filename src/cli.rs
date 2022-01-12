#[derive(Debug, structopt::StructOpt)]
#[structopt(about)]
pub struct Cli {
    #[structopt(name = "config file", short = "f", long = "file", parse(from_os_str))]
    pub(crate) config_file: std::path::PathBuf,
}
