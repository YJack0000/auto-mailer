use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub email: String,
    pub password: String,
    pub title: String,
    pub csv_file_path: String,
    pub template_file_path: String,
}

pub fn read_config(config_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_str = fs::read_to_string(config_path)?;
    parse_config(&config_str)
}

pub fn parse_config(config_data: &str) -> Result<Config, Box<dyn Error>> {
    let config: Config = toml::from_str(config_data)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config_toml = r#"
            email = "user@example.com"
            password = "secret"
            title = "The title of the email"
            csv_file_path = "data.csv"
            template_file_path = "template.txt"
        "#;

        let config = parse_config(config_toml).unwrap();

        assert_eq!(config.email, "user@example.com");
        assert_eq!(config.password, "secret");
        assert_eq!(config.title, "The title of the email");
        assert_eq!(config.csv_file_path, "data.csv");
        assert_eq!(config.template_file_path, "template.txt");
    }
}
