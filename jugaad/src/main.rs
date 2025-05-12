use ratatui::{Terminal, backend::CrosstermBackend, widgets::{Block, Borders, Paragraph}, text::{Span}};
use crossterm::{terminal, event, event::KeyCode};
use std::io::{self};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Clear the terminal screen
    terminal.clear()?;

    // Build the UI
    terminal.draw(|f| {
        let size = f.area();
        let block = Block::default().borders(Borders::ALL).title("GitHub CLI Authentication");
        f.render_widget(block, size);

        // User input section (PAT prompt)
        let paragraph = Paragraph::new(Span::raw("Please enter your GitHub PAT:"))
            .block(Block::default().borders(Borders::ALL).title("GitHub Authentication"));
        f.render_widget(paragraph, size);
    })?;

    // Handle user input for PAT
    let mut pat_input = String::new();
    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Enter => {
                        break;  // User hits Enter to submit
                    },
                    KeyCode::Char(c) => {
                        pat_input.push(c);  // Add character to input
                    },
                    KeyCode::Backspace => {
                        pat_input.pop();  // Remove last character on Backspace
                    },
                    _ => {}
                }
            }
        }
        // Optional: Display current input
        terminal.draw(|f| {
            let size = f.area();
            let input_box = Paragraph::new(Span::raw(&pat_input))
                .block(Block::default().borders(Borders::ALL).title("Your PAT"));
            f.render_widget(input_box, size);
        })?;
    }

    // Handle the PAT here (call your authentication function with the PAT)
    println!("Your PAT is: {}", pat_input);

    // Clean up
    terminal::disable_raw_mode()?;
    Ok(())
}
