use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{cursor, terminal, QueueableCommand};

fn main() {
    println!("Hello Client");
    let mut stdout = stdout();
    // stdout.queue(terminal::Clear(terminal::ClearType::All));
    stdout.queue(cursor::MoveTo(5, 5)).unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_secs(5));
}
