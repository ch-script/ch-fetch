// Module for ASCII art control, its designed to manage the renders for logos

// Useful structures for the upcoming future
//struct Color(u8, u8, u8);

pub struct AsciiArt {
    //custom: Option<String>, //this is for the route of a personalized one
    //distro: String,
    fallback: &'static str,
}

impl AsciiArt {
    pub fn new() -> Self {
        AsciiArt {
            fallback: include_str!("../assets/ascii/fallback.txt"),
        }
    }
    pub fn get(&self) -> &str {
        self.fallback
    }
}