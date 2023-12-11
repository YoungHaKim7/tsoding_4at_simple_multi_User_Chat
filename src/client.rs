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
    let mut stdout = stdout();
    let (mut w, mut h) = terminal::size().unwrap();
    let mut bar = "-".repeat(w as usize);
    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    bar = "-".repeat(w as usize);
                }
                Event::Key(_event) => {}
                Event::FocusGained => {}
                Event::FocusLost => {}
                Event::Mouse(_) => {}
                Event::Paste(_) => {}
            }
        }
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(MoveTo(0, h - 2)).unwrap();
        stdout.write(bar.as_bytes()).unwrap();
        stdout.queue(MoveTo(0, h - 1)).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(5));
    }

    let label = b"urmom";
    stdout
        .queue(MoveTo(w / 2 - label.len() as u16 / 2, h / 2))
        .unwrap();
    stdout.write(label).unwrap();
    stdout.flush().unwrap();
}
