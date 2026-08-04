// Main Program for the CLI simple fetch CH-FETCH v0
// it has almost nothing done yet, it just shows things

// Modules

mod ascii;
mod system;
mod sysprovider;
mod terminalrenderer;
mod theme;
mod flags;
mod config;

// Use Modules

use ascii::AsciiArt;
use system::InformationManager;
use sysprovider::SysinfoManager;
use terminalrenderer::terminal_output;
use clap::Parser;
use flags::Flags;
use config::Config;

// Imports

// uncomment to enable dictionaries
//use std::collections::HashMap;


// Main Program
fn main() {

    let flags = Flags::parse();
    flags.handle(); // this handles flags, such as "ch-fetch --version" and that kind of things
    
    let config = Config::load(flags.load_config.as_deref());

    let sys_mng = SysinfoManager::new();
    let data = InformationManager::new(&sys_mng);
    let ascii_mng = AsciiArt::new();

    terminal_output(&data, &ascii_mng, &config);
}

// Useful structures


// Custom Conf Override

