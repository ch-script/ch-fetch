// Module for ASCII art control, its designed to manage the renders for logos

// Imports 

use include_dir::{include_dir, Dir};

// Useful structures for the upcoming future
//struct Color(u8, u8, u8);

static GENERICS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/ascii/generics"); //now i manage other ascii arts with include_dir macro


pub struct AsciiArt {
    pub allow_customs: bool,
    pub custom: &'static str,
    pub fallback: &'static str,
}

impl AsciiArt {
    pub fn new() -> Self {
        AsciiArt {
            allow_customs: false,
            custom: include_str!("../assets/ascii/custom.txt"),
            fallback: include_str!("../assets/ascii/fallback.txt"),
        }
    }

    pub fn get(&self, distro_name: &str) -> &'static str {
        let normalized_distro_name = distro_name.to_lowercase().replace(' ', "_");
        
        if self.allow_customs {
            return self.custom;
        }


        let file_name = format!("{}.txt", normalized_distro_name);

        if let Some(file) = GENERICS_DIR.get_file(&file_name) { // if it isnt it looks for the distro
            if let Some(content) = file.contents_utf8() {
                    return content;
                }
        }

        self.fallback // if everything fails it prints fallback
    }
}