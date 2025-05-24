use colored::*;

fn main() {
    let sentence = vec![
        "Rust".red(),
        "is".green(),
        "fast,".blue(),
        "safe,".yellow(),
        "and".magenta(),
        "fun!".cyan(),
    ];

    for word in sentence {
        print!("{} ", word);
    }

    println!(); // for a newline
}
