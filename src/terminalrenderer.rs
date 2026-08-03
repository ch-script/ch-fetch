// Module that controls the render that's shown on terminal
// I use crossterm crate for Terminal Interface, see more: https://docs.rs/crossterm/latest/crossterm/index.html 

// Imports

use crossterm::{
    cursor::{MoveTo, position},
    event::{self, Event, KeyCode, KeyEventKind},
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{stdout, BufWriter, Write};
use std::time::Duration;
use std::process::Command;

use std::thread::sleep;
use std::thread;

// Modules

//mod theme;

// Use Modules

use crate::InformationManager;
use crate::AsciiArt;
use std::cmp::max;
use crate::theme::Theme;

// Main Script

pub fn terminal_output(data: &InformationManager, ascii: &AsciiArt) {

    let theme = Theme::default(); // default colors for the time being
    //enable_raw_mode();
    let mut principal_stdout = BufWriter::new(stdout()); // crossform migration!

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

        let _ = queue!(principal_stdout, SetForegroundColor(theme.ascii_color), Print(format!("{}", ascii_part))); 
        let _ = queue!(principal_stdout, Print("          ")); // padding in a sense
        let _ = queue!(principal_stdout, SetForegroundColor(Color::White), Print(format!("{}\n", info_part)));
    }

    let _ = principal_stdout.flush();


    // garbage code to a future animated implementation, yk... main thread is required to run this, but if that's activated you cannot access bash..
    // maybe trying to make my own shell'll be the way to victory? who knows .. . ..

    // let mut stdoutControl = BufWriter::new(stdout());

    // let text = "Test";

    // queue!(stdoutControl, SetForegroundColor(Color::Red), Print(text));
    // queue!(stdoutControl, SetForegroundColor(Color::White), Print(": something\n"));

    // stdoutControl.flush();

    // let (x,y) = position().unwrap();

    // let handle = thread::spawn(move || -> Result<(), std::io::Error> {
    //     let mut stdout_test = BufWriter::new(stdout());

    //     for i in 0..9 {
    //         let sum = x + (i as u16);

    //         queue!(stdout_test, Clear(ClearType::CurrentLine), MoveTo(sum, y), Print("animation"))?;
    //         stdout_test.flush()?;
    //         sleep(Duration::from_millis(100));
    //     }
    //     Ok(())
    // });

    // let _ = handle.join();

}