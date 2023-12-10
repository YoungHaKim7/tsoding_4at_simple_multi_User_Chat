use std::io::{stdout, Error};

use crossterm::{cursor, QueueableCommand};

fn main() -> Result<(), Error> {
    println!("Hello Client");
    let mut stdout = stdout();
    stdout.queue(cursor::MoveTo(5, 5));
    Ok(())
}
