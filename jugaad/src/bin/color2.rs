use crossterm::style::{Color, Stylize};

fn main() {
    // Foreground color
    println!("{}", "Red text".with(Color::Red));

    // Background color
    println!("{}", "Text with blue background".on(Color::Blue));

    // Combined styles
    println!("{}", "Green bold text on yellow".green().bold().on_yellow());
}
