use std::io::{stdout, Write};

use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    QueueableCommand,
};

fn main() {
    println!("Hello Client");
    let mut stdout = stdout();
    let _ = stdout.queue(Clear(ClearType::All));
    stdout.queue(cursor::MoveTo(5, 5)).unwrap();
    stdout.flush().unwrap();
}
