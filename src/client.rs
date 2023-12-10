use std::{io::stdout, thread, time::Duration};

use crossterm::{cursor, QueueableCommand};

fn main() {
    println!("Hello Client");
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(5, 5)).unwrap();
    thread::sleep(Duration::from_secs(5));
}
