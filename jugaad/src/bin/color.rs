use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout();

    // Print red text
    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("[auth] "),
        ResetColor
    ).unwrap();
    println!("Authenticating with GitHub via SSH...");
    // Print green text
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("This is green text\n"),
        ResetColor
    ).unwrap();

    // Print blue text
    execute!(
        stdout,
        SetForegroundColor(Color::Blue),
        Print("This is blue text\n"),
        ResetColor
    ).unwrap();
}
