// This module handles the declarative configuration

// Imports

// Use Imports 

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// this all available modules/blocks that the user can place in their layout for now
#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Module {
    Ascii,
    Title,
    Separator,
    Os,
    Kernel,
    Uptime,
    Terminal,
    Shell,
    Cpu,
    Gpu,
    Ram,
    Disk,
    Empty,
}

/// horizontal padding configuration section
#[derive(Debug, Deserialize)]
pub struct PaddingConfig {
    #[serde(default = "default_column_gap")]
    pub column_gap: usize,

    #[serde(default = "default_label_width")]
    pub label_width: usize,

    #[serde(default)]
    pub indent_info: usize,

    #[serde(default)]
    pub indent_ascii: usize,
}

fn default_column_gap() -> usize { 5 }
fn default_label_width() -> usize { 10 }

impl Default for PaddingConfig {
    fn default() -> Self {
        Self {
            column_gap: default_column_gap(),
            label_width: default_label_width(),
            indent_info: 0,
            indent_ascii: 0,
        }
    }
}

/// Layout section in config
#[derive(Debug, Deserialize)]
pub struct LayoutConfig {
    pub columns: Vec<Vec<Module>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub padding: PaddingConfig,
    pub layout: LayoutConfig,
}

impl Default for Config {
    /// Fallback or default whatever u want to call it... i mean its author's conf (me)
    fn default() -> Self {
        Self {
            padding: PaddingConfig::default(),
            layout: LayoutConfig {
                columns: vec![
                    // Column 0
                    vec![Module::Ascii],
                    // Column 1
                    vec![
                        Module::Title,
                        Module::Separator,
                        Module::Os,
                        Module::Kernel,
                        Module::Uptime,
                        Module::Terminal,
                        Module::Shell,
                        Module::Empty,
                        Module::Cpu,
                        Module::Gpu,
                        Module::Ram,
                        Module::Disk,
                    ],
                ],
            },
        }
    }
}

impl Config {
    /// Loads config from a custom path
    pub fn load(custom_path: Option<&str>) -> Self {
        let path = match custom_path {
            Some(p) => p.to_string(),
            None => match dirs::config_dir() {
                Some(mut p) => {
                    p.push("ch-fetch/config.toml");
                    p.to_str().unwrap_or("").to_string()
                }
                None => return Self::default(),
            },
        };

        if !Path::new(&path).exists() {
            return Self::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => match toml::from_str::<Config>(&content) {
                Ok(config) => config,
                Err(err) => {
                    eprintln!("WARNING! Error parsing config file: {}. Using defaults.", err);
                    Self::default()
                }
            },
            Err(_) => Self::default(),
        }
    }

    pub fn create_default_file() -> Result<String, std::io::Error> {
        let config_dir = match dirs::config_dir() {
            Some(mut path) => {
                path.push("ch-fetch");
                path
            }
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find XDG config directory",
                ))
            }
        };

        fs::create_dir_all(&config_dir)?;

        let file_path = config_dir.join("config.toml");

        // Template (to be moved and includestr WIP)
        let template = r#"# CH-FETCH Configuration File
# Edit this file to customize your fetch layout

[padding]
# Horizontal spacing between columns in characters
column_gap = 5

# Alignment width for label names
label_width = 10

# Indentation for system info text
indent_info = 0

# Indentation for ASCII art
indent_ascii = 0

[layout]
# Define your columns as a list of lists.
# Available modules for now (chill there'll be more): "ascii", "title", "separator", "os", "kernel", "uptime",
# "terminal", "shell", "cpu", "gpu", "ram", "disk", "empty"
columns = [
    ["ascii"],
    [
        "title",
        "separator",
        "os",
        "kernel",
        "uptime",
        "terminal",
        "shell",
        "cpu",
        "gpu",
        "ram",
        "disk"
    ]
]
"#;

        fs::write(&file_path, template)?;
        Ok(file_path.to_string_lossy().into_owned())
    }
}