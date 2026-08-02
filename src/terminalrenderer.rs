// Module that controls the render that's shown on terminal
// For the moment is empty, and it uses simple prints to work... but with the new updates it will include a Terminal Manipulation Library

// Modules

// Use Modules

use crate::InformationManager;
use crate::AsciiArt;
use std::cmp::max;

// Main Script

pub fn terminal_output(data: &InformationManager, ascii: &AsciiArt) {
    let ascii_lines: Vec<&str> = ascii.get().lines().collect();
    
    let disk_raw = data.memory().trim();
    let disk_lines: Vec<&str> = disk_raw.lines().collect();

    let mut info_lines = vec![
        format!("User:     {}", data.user()),
        format!("Host:     {}", data.host()),
        "-----------------------------".to_string(),
        format!("Distro:   {}", data.os()),
        format!("Kernel:   {}", data.kernel()),
        format!("Uptime:   {}", data.uptime()),
        format!("Terminal: {}", data.terminal()),
        format!("Shell:    {}", data.shell()),
        "".to_string(),
        format!("CPU:      {}", data.cpu()),
        format!("GPU:      {}", data.gpu()),
        format!("RAM:      {}", data.ram()),
    ];

    for d in disk_lines {
        info_lines.push(d.to_string());
    }

    let total_lines = max(ascii_lines.len(), info_lines.len());

    let ascii_width = ascii_lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    for i in 0..total_lines {
        let ascii_part = if i < ascii_lines.len() {
            let line = ascii_lines[i];
            let char_count = line.chars().count();
            let pad = " ".repeat(ascii_width.saturating_sub(char_count));
            format!("{}{}", line, pad)
        } else {
            " ".repeat(ascii_width)
        };

        let info_part = if i < info_lines.len() {
            &info_lines[i]
        } else {
            ""
        };

        println!("{}   {}", ascii_part, info_part);
    }
}