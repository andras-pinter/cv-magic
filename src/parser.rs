use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub template: std::path::PathBuf,
    #[serde(default)]
    pub globals: HashMap<String, String>,
    #[serde(default)]
    pub sections: HashMap<String, HashMap<String, toml::Value>>,
}

impl TryFrom<&'static str> for Config {
    type Error = toml::de::Error;

    fn try_from(raw: &'static str) -> Result<Self, Self::Error> {
        toml::from_str(raw)
    }
}

impl Config {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> crate::Result<Self> {
        let mut buffer = Vec::new();

        let mut file = std::fs::File::open(path)?;
        file.read_to_end(&mut buffer)?;

        Ok(toml::from_slice(&buffer)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Config;

    #[test]
    fn test_template() {
        let config = Config::try_from(
            r#"
        template = "test.tpl"
        "#,
        );

        assert!(
            config.is_ok(),
            "Config should be parsable: {:?}",
            config.err()
        );
        assert_eq!(config.unwrap().template.to_str(), Some("test.tpl"))
    }

    #[test]
    fn test_globals() {
        let config = Config::try_from(
            r#"
        template = "test.tpl"

        [globals]
        title = "my-cv"
        key = "value"
        "#,
        );

        assert!(
            config.is_ok(),
            "Config should be parsable: {:?}",
            config.err()
        );
        let config = config.unwrap();
        assert_eq!(config.globals.len(), 2);
        assert_eq!(config.globals.get("title"), Some(&"my-cv".to_string()));
        assert_eq!(config.globals.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_sections() {
        let config = Config::try_from(
            r#"
        template = "test.tpl"

        [sections.first_job]
        year = "2007"
        position = "junior"
        job = "software engineer"

        [sections.second_job]
        year = "2010"
        position = "senior"
        job = "software engineer"

        [sections.technologies]
        languages = ["python", "php", "rust"]
        "#,
        );

        assert!(
            config.is_ok(),
            "Config should be parsable: {:?}",
            config.err()
        );
        let config = config.unwrap();
        assert_eq!(config.sections.len(), 3);
        assert!(config.sections.get("first_job").is_some());
        assert!(config.sections.get("second_job").is_some());
        assert!(config.sections.get("technologies").is_some());

        let first_job = config.sections.get("first_job").unwrap();
        assert_eq!(
            first_job.get("year").cloned(),
            Some(toml::Value::String("2007".to_string()))
        );
        assert_eq!(
            first_job.get("position").cloned(),
            Some(toml::Value::String("junior".to_string()))
        );
        assert_eq!(
            first_job.get("job").cloned(),
            Some(toml::Value::String("software engineer".to_string()))
        );

        let second_job = config.sections.get("second_job").unwrap();
        assert_eq!(
            second_job.get("year").cloned(),
            Some(toml::Value::String("2010".to_string()))
        );
        assert_eq!(
            second_job.get("position").cloned(),
            Some(toml::Value::String("senior".to_string()))
        );
        assert_eq!(
            second_job.get("job").cloned(),
            Some(toml::Value::String("software engineer".to_string()))
        );

        let technologies = config.sections.get("technologies").unwrap();
        assert_eq!(
            technologies.get("languages").cloned(),
            Some(toml::Value::Array(vec![
                toml::Value::String("python".to_string()),
                toml::Value::String("php".to_string()),
                toml::Value::String("rust".to_string()),
            ]))
        )
    }

    #[test]
    fn test_mandatory_template() {
        let config = Config::try_from("");

        assert!(config.is_err());
    }
}
