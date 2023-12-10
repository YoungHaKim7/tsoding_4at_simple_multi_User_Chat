use std::io::stdout;

use crossterm::{cursor, QueueableCommand};

fn main() {
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(5, 5));
    println!("Hello Client");
}
