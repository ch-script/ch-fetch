// Module that controls the render that's shown on terminal
// I use crossterm crate for Terminal Interface, see more: https://docs.rs/crossterm/latest/crossterm/index.html 

// Imports

use crossterm::{
    //cursor::{MoveTo, position},
    //event::{self, Event, KeyCode, KeyEventKind},
    queue,
    style::{Print, SetForegroundColor},
    //terminal::{
    //    disable_raw_mode, enable_raw_mode, Clear, ClearType,
    //    EnterAlternateScreen, LeaveAlternateScreen,
    //},
};
use std::io::{stdout, BufWriter, Write};
//use std::time::Duration;
//use std::process::Command;

//use std::thread::sleep;
//use std::thread;

// Modules

//mod theme;

// Use Modules

use crate::InformationManager;
use crate::AsciiArt;
use std::cmp::max;
use crate::theme::Theme;

// Main Script

pub enum InfoRow<'a> {
    Title { user: &'a str, host: &'a str },
    Separator,
    Item { label: &'a str, value: &'a str },
    Empty,
}

pub fn terminal_output(data: &InformationManager, ascii: &AsciiArt) {

    let theme = Theme::default(); // default colors for the time being
    //enable_raw_mode();
    let mut principal_stdout = BufWriter::new(stdout()); // crossform migration!

    let ascii_content = ascii.get(data.os());
    let ascii_lines: Vec<&str> = ascii_content.lines().collect();
    
    let disk_raw = data.memory().trim();
    let disk_lines: Vec<&str> = disk_raw.lines().collect();

    let mut info_lines: Vec<InfoRow> = vec![
        InfoRow::Title { user: data.user(), host: data.host() },
        InfoRow::Separator,
        InfoRow::Item { label: "Distro:", value: data.os() },
        InfoRow::Item { label: "Kernel:", value: data.kernel() },
        InfoRow::Item { label: "Uptime:", value: data.uptime() },
        InfoRow::Item { label: "Terminal:", value: data.terminal() },
        InfoRow::Item { label: "Shell:", value: data.shell() },
        InfoRow::Empty,
        InfoRow::Item { label: "CPU:", value: data.cpu() },
        InfoRow::Item { label: "GPU:", value: data.gpu() },
        InfoRow::Item { label: "RAM:", value: data.ram() },
    ];

    for d in &disk_lines {
        info_lines.push(InfoRow::Item { 
            label: "Disk:", 
            value: d 
        });
    }

    let total_lines = max(ascii_lines.len(), info_lines.len());

    let ascii_width = ascii_lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    for i in 0..total_lines {
        if i < ascii_lines.len() {
            let line = ascii_lines[i];
            let _ = queue!(principal_stdout, SetForegroundColor(theme.ascii_color), Print(line));
            
            let pad_len = ascii_width.saturating_sub(line.chars().count());
            if pad_len > 0 {
                let _ = queue!(principal_stdout, Print(format_args!("{:width$}", "", width = pad_len)));
            }
        } else {
            let _ = queue!(principal_stdout, Print(format_args!("{:width$}", "", width = ascii_width)));
        }

        let _ = queue!(principal_stdout, Print("          "));

        if i < info_lines.len() {
            match info_lines[i] {
                InfoRow::Title { user, host } => {
                    let _ = queue!(principal_stdout, SetForegroundColor(theme.title_color), Print(user));
                    let _ = queue!(principal_stdout, SetForegroundColor(theme.separator_color), Print("@"));
                    let _ = queue!(principal_stdout, SetForegroundColor(theme.title_color), Print(host));
                }
                InfoRow::Separator => {
                    let _ = queue!(
                        principal_stdout, 
                        SetForegroundColor(theme.separator_color), 
                        Print("-----------------------------")
                    );
                }
                InfoRow::Item { label, value } => {
                    let _ = queue!(
                        principal_stdout, 
                        SetForegroundColor(theme.label_color), 
                        Print(format_args!("{:<10}", label))
                    );
                    let _ = queue!(
                        principal_stdout, 
                        SetForegroundColor(theme.value_color), 
                        Print(value)
                    );
                }
                InfoRow::Empty => {}
            }
        }

        let _ = queue!(principal_stdout, Print("\n"));
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