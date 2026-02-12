use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VshConfig {
    /// Default syntax preference
    pub default_syntax: Option<String>,

    /// Error handling options
    pub show_suggestions: bool,
    pub show_examples: bool,
    pub explain_errors: bool,

    /// Prompt customization
    pub prompt: Option<String>,

    /// Color scheme
    pub color_scheme: Option<String>,
}

impl Default for VshConfig {
    fn default() -> Self {
        Self {
            default_syntax: Some("adaptive".to_string()),
            show_suggestions: true,
            show_examples: true,
            explain_errors: true,
            prompt: Some("vsh$ ".to_string()),
            color_scheme: Some("default".to_string()),
        }
    }
}

impl VshConfig {
    /// Load config from .vshrc file
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            if let Ok(contents) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str(&contents) {
                    return config;
                }
            }
        }

        // Return default if can't load
        Self::default()
    }

    /// Save config to .vshrc file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(config_path, contents)?;
        Ok(())
    }

    /// Get the path to the config file
    fn config_path() -> PathBuf {
        dirs::home_dir()
            .map(|mut p| {
                p.push(".vshrc");
                p
            })
            .unwrap_or_else(|| PathBuf::from(".vshrc"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VshConfig::default();
        assert!(config.show_suggestions);
        assert!(config.show_examples);
        assert_eq!(config.default_syntax, Some("adaptive".to_string()));
    }

    #[test]
    fn test_serialize_config() {
        let config = VshConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("show_suggestions"));
    }
}
