use std::net::{IpAddr, SocketAddr};

#[derive(Debug, structopt::StructOpt)]
#[structopt(about)]
pub struct Cli {
    #[structopt(
        name = "ip address",
        short = "a",
        long = "address",
        default_value = "0.0.0.0"
    )]
    ip: String,
    #[structopt(name = "port", short = "p", long = "port", default_value = "80")]
    port: u16,
    #[structopt(name = "config file", short = "f", long = "file", parse(from_os_str))]
    pub(crate) config_file: std::path::PathBuf,
}

impl Cli {
    pub fn addr(&self) -> crate::Result<SocketAddr> {
        Ok(SocketAddr::from((self.ip.parse::<IpAddr>()?, self.port)))
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use structopt::StructOpt;

    #[test]
    fn test_default_values() {
        let args = Cli::from_iter_safe(&["", "-f", "test.toml"]);
        assert!(
            args.is_ok(),
            "command line parsing should be ok: {:?}",
            args.err()
        );
        assert_eq!(args.unwrap().config_file.to_str(), Some("test.toml"));
    }

    #[test]
    fn test_address() {
        let args = Cli::from_iter_safe(&["", "-f", "test.toml", "-a", "0.0.0.0", "-p", "8080"]);
        let expected = super::SocketAddr::from(([0, 0, 0, 0], 8080));
        assert!(
            args.is_ok(),
            "command line parsing should be ok: {:?}",
            args.err()
        );

        let addr = args.unwrap().addr();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap(), expected);
    }

    #[test]
    fn test_config_file_path_must_provided() {
        let args = Cli::from_iter_safe(&[""]);
        assert!(args.is_err());
    }

    #[test]
    fn test_invalid_address() {
        let args = Cli::from_iter_safe(&["", "-f", "test.toml", "-a", "1.2.3", "-p", "8080"]);
        assert!(
            args.is_ok(),
            "command line parsing should be ok: {:?}",
            args.err()
        );

        let addr = args.unwrap().addr();
        assert!(addr.is_err());
    }

    #[test]
    fn test_invalid_port() {
        let args = Cli::from_iter_safe(&["", "-f", "test.toml", "-a", "0.0.0.0", "-p", "alma"]);
        assert!(args.is_err());
    }
}
