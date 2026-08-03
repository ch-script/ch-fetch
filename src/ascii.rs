// Module for ASCII art control, its designed to manage the renders for logos

// Imports 

use std::fs;
use std::path::Path;

// Useful structures for the upcoming future
//struct Color(u8, u8, u8);

pub struct AsciiArt {
    pub allow_customs: bool,
    pub custom_dir: &'static str,
    pub distros_dir: &'static str,
    pub fallback: &'static str,
}

impl AsciiArt {
    pub fn new() -> Self {
        AsciiArt {
            allow_customs: false,
            custom_dir: "assets/ascii/custom/",
            distros_dir: "assets/ascii/distros/",
            fallback: include_str!("../assets/ascii/fallback.txt"),
        }
    }

    pub fn get(&self, distro_name: &str) -> String {
        let normalized_distro_name = distro_name.to_lowercase();

        if self.allow_customs { // if customs is activated
            let custom_path = format!("{}/ascii.txt", self.custom_dir);
            if Path::new(&custom_path).exists() {
                if let Ok(content) = fs::read_to_string(&custom_path) {
                    return content;
                }
            }
        }

        let distro_path = format!("{}/{}.txt", self.distros_dir, normalized_distro_name); // if it isnt it looks for the distro
        if Path::new(&distro_path).exists() {
            if let Ok(content) = fs::read_to_string(&distro_path) {
                return content;
            }
        }

        self.fallback.to_string() // if everything fails it prints fallback
    }
}