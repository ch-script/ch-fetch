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
// use std::cmp::max;
use crate::theme::Theme;
use crate::config::{Config, Module};

// Main Script

enum RenderLine {
    Text { indent: usize, label: String, value: String },
    Title { indent: usize, user: String, host: String },
    Separator { indent: usize },
    Raw { indent: usize, text: String },
    Empty,
}

impl RenderLine {
    fn len(&self) -> usize {
        match self {
            RenderLine::Text { indent, label, value } => indent + label.chars().count() + value.chars().count(),
            RenderLine::Title { indent, user, host } => indent + user.chars().count() + 1 + host.chars().count(),
            RenderLine::Separator { indent } => indent + 29,
            RenderLine::Raw { indent, text } => indent + text.chars().count(),
            RenderLine::Empty => 0,
        }
    }
}

pub fn terminal_output(data: &InformationManager, ascii: &AsciiArt, config: &Config) {

    let theme = Theme::default(); // default colors for the time being
    //enable_raw_mode();
    let mut principal_stdout = BufWriter::new(stdout()); // crossform migration!

    let ascii_content = ascii.get(data.os());
    let ascii_lines: Vec<&str> = ascii_content.lines().collect();
    
    let disk_raw = data.memory().trim();
    let disk_lines: Vec<&str> = disk_raw.lines().collect();

    let label_w = config.padding.label_width;
    let info_indent = config.padding.indent_info;
    let ascii_indent = config.padding.indent_ascii;

    let make_label = |raw: &str| -> String {
        format!("{:width$}", raw, width = label_w)
    };

    let mut columns_render: Vec<Vec<RenderLine>> = Vec::new();

    for col_modules in &config.layout.columns {
        let mut column_lines: Vec<RenderLine> = Vec::new();

        for module in col_modules {
            match module {
                Module::Ascii => {
                    for line in &ascii_lines {
                        column_lines.push(RenderLine::Raw { indent: ascii_indent, text: line.to_string() });
                    }
                }
                Module::Title => {
                    column_lines.push(RenderLine::Title {
                        indent: info_indent,
                        user: data.user().to_string(),
                        host: data.host().to_string(),
                    });
                }
                Module::Separator => {
                    column_lines.push(RenderLine::Separator { indent: info_indent });
                }
                Module::Os => column_lines.push(RenderLine::Text { indent: info_indent, label: make_label("Distro:"), value: data.os().into() }),
                Module::Kernel => column_lines.push(RenderLine::Text { indent: info_indent, label: make_label("Kernel:"), value: data.kernel().into() }),
                Module::Uptime => column_lines.push(RenderLine::Text { indent: info_indent, label: make_label("Uptime:"), value: data.uptime().into() }),
                Module::Terminal => column_lines.push(RenderLine::Text { indent: info_indent, label: make_label("Terminal:"), value: data.terminal().into() }),
                Module::Shell => column_lines.push(RenderLine::Text { indent: info_indent, label: make_label("Shell:"), value: data.shell().into() }),
                Module::Cpu => column_lines.push(RenderLine::Text { indent: info_indent, label: make_label("CPU:"), value: data.cpu().into() }),
                Module::Gpu => column_lines.push(RenderLine::Text { indent: info_indent, label: make_label("GPU:"), value: data.gpu().into() }),
                Module::Ram => column_lines.push(RenderLine::Text { indent: info_indent, label: make_label("RAM:"), value: data.ram().into() }),
                Module::Disk => {
                    for d in &disk_lines {
                        column_lines.push(RenderLine::Text { indent: info_indent, label: make_label("Disk:"), value: d.to_string() });
                    }
                }
                Module::Empty => column_lines.push(RenderLine::Empty),
            }
        }

        columns_render.push(column_lines);
    }

    let col_widths: Vec<usize> = columns_render.iter().map(|col| col.iter().map(|line| line.len()).max().unwrap_or(0)).collect();

    let max_rows = columns_render.iter().map(|col| col.len()).max().unwrap_or(0);

    //println!("DEBUG col_widths: {:?}", col_widths);

    for row in 0..max_rows {
        for (col_idx, column) in columns_render.iter().enumerate() {
            let width = col_widths[col_idx];
            let is_last_col = col_idx == columns_render.len() - 1;

            if let Some(line) = column.get(row) {
                let printed_len = line.len();

                match line {
                    RenderLine::Title { indent, user, host } => {
                        if *indent > 0 { let _ = queue!(principal_stdout, Print(format_args!("{:width$}", "", width = indent))); }
                        let _ = queue!(principal_stdout, SetForegroundColor(theme.title_color), Print(user));
                        let _ = queue!(principal_stdout, SetForegroundColor(theme.separator_color), Print("@"));
                        let _ = queue!(principal_stdout, SetForegroundColor(theme.title_color), Print(host));
                    }
                    RenderLine::Separator { indent } => {
                        if *indent > 0 { let _ = queue!(principal_stdout, Print(format_args!("{:width$}", "", width = indent))); }
                        let _ = queue!(
                            principal_stdout,
                            SetForegroundColor(theme.separator_color),
                            Print("-----------------------------")
                        );
                    }
                    RenderLine::Text { indent, label, value } => {
                        if *indent > 0 { let _ = queue!(principal_stdout, Print(format_args!("{:width$}", "", width = indent))); }
                        let _ = queue!(principal_stdout, SetForegroundColor(theme.label_color), Print(label));
                        let _ = queue!(principal_stdout, SetForegroundColor(theme.value_color), Print(value));
                    }
                    RenderLine::Raw { indent, text } => {
                        if *indent > 0 { let _ = queue!(principal_stdout, Print(format_args!("{:width$}", "", width = indent))); }
                        let _ = queue!(principal_stdout, SetForegroundColor(theme.ascii_color), Print(text));
                    }
                    RenderLine::Empty => {}
                }

                if !is_last_col {
                    let pad = width.saturating_sub(printed_len) + config.padding.column_gap;
                    let _ = queue!(principal_stdout, Print(format_args!("{:width$}", "", width = pad)));
                }
            } else {
                if !is_last_col {
                    let pad = width + config.padding.column_gap;
                    let _ = queue!(principal_stdout, Print(format_args!("{:width$}", "", width = pad)));
                }
            }
        }

        let _ = queue!(principal_stdout, Print("\n"));
    }

    let _ = principal_stdout.flush();
}



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

