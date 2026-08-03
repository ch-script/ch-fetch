// Module that manages the theme

use crossterm::style::Color;

pub struct Theme {
    pub ascii_color: Color,
    pub title_color: Color,
    pub label_color: Color,
    pub value_color: Color,
    pub separator_color: Color,
}

impl Default for Theme { // using a default theme for the time being
    fn default() -> Self {
        Self {
            ascii_color: Color::Cyan,
            title_color: Color::Yellow,
            label_color: Color::Red,
            value_color: Color::White,
            separator_color: Color::DarkGrey,
        }
    }
}