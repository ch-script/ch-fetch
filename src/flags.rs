// This module handles flags for the program configuration

// Imports

// Use Imports

use clap::Parser;
use crate::config::Config;

// declarative fetch
#[derive(Parser, Debug)]
#[command(
    name = "ch-fetch",
    author,       // WHO did this !?! who is this kid
    version,      // idk its the standart to know the version ur on aint it
    about,        // so you have curiosity?
    long_about = None
)]

pub struct Flags {
    /// it creates a config file, here: ~/.config/ch-fetch/
    #[arg(short, long)]
    pub create_config: bool,

    /// send ur personalized conf archive
    #[arg(short, long, value_name = "PATH")]
    pub load_config: Option<String>,

    #[arg(long)]
    pub about: bool,

}

impl Flags {
    // what to do based on the later flags
    pub fn handle(&self) {
        if self.create_config {
            match Config::create_default_file() {
                Ok(path) => {
                    println!("Config file created successfully at: {}", path);
                    println!("LET THE LARP BEGIN!!");
                }
                Err(err) => {
                    eprintln!("Failed to create config file: {} :c", err);
                }
            }
            std::process::exit(0);
        }

        if let Some(path) = &self.load_config {
            println!("Loading ur incredible and wonderful personalization from: {}", path);
        }

        if self.about {
            println!("Ch-fetch was created to be a nice cute little fetch based on a declarative basis.");
            println!("Created with love by a Nix enthusiast.");
            std::process::exit(0);
        }
    }
}