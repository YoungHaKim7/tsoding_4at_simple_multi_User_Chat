use std::io::{stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

fn main() {
    println!("Hello from Client");
    let mut stdout = stdout();
    let (width, height) = terminal::size().unwrap();
    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.queue(MoveTo(width / 2, height / 2));
    let _ = stdout.write(b"wrmom");
    stdout.flush().unwrap();
}
