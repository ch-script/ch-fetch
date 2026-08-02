// Main Program for the CLI simple fetch CH-FETCH v0
// it has almost nothing done yet, it just shows things

// Modules

mod ascii;
mod system;
mod sysprovider;
mod terminalrenderer;

// Use Modules

use ascii::AsciiArt;
use system::InformationManager;
use sysprovider::SysinfoManager;

use terminalrenderer::terminal_output;

// Imports

// uncomment to enable dictionaries
//use std::collections::HashMap;


// Main Program
fn main() {

    let sys_mng = SysinfoManager::new();
    let data = InformationManager::new(&sys_mng);
    let ascii_mng = AsciiArt::new();


    terminal_output(&data,&ascii_mng);
}

// Useful structures


// Custom Conf Override

