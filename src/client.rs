use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{self, MoveTo},
    event::{poll, read, Event},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

fn main() {
    let (mut w, mut h) = terminal::size().unwrap();
    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                }
                Event::Key(evet) => todo!(),
                Event::FocusGained => todo!(),
                Event::FocusLost => todo!(),
                Event::Mouse(_) => todo!(),
                Event::Paste(_) => todo!(),
            }
        }
        stdout().queue(Clear(ClearType::All)).unwrap();
    }
    println!("Hello from Client");
    let mut stdout = stdout();
    let (width, height) = terminal::size().unwrap();
    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.queue(MoveTo(width / 2, height / 2));
    let _ = stdout.write("wrmom".as_bytes()).unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_secs(3));
}
