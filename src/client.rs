use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode, KeyModifiers},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn chat_window(stdout: &mut impl Write, chat: &[String], boundary: Rect) {
    let n = chat.len();
    let m = n.checked_sub(boundary.h).unwrap_or(0);

    for (dy, line) in chat.iter().skip(m).enumerate() {
        stdout
            .queue(MoveTo(boundary.x as u16, (boundary.y + dy) as u16))
            .unwrap();
        let bytes = line.as_bytes();
        stdout
            .write(bytes.get(0..boundary.w).unwrap_or(bytes))
            .unwrap();
    }
}

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    let (mut w, mut h) = terminal::size().unwrap();
    let bar_char = "═";
    let mut bar = bar_char.repeat(w as usize);
    let mut quit = false;
    let mut prompt = String::new();
    let mut chat = Vec::new();
    while !quit {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    bar = bar_char.repeat(w as usize);
                }
                Event::Key(event) => match event.code {
                    KeyCode::Char(x) => {
                        if x == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                            quit = true;
                        } else {
                            prompt.push(x)
                        }
                    }
                    KeyCode::Enter => {
                        chat.push(prompt.clone());
                        prompt.clear();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        stdout.queue(Clear(ClearType::All)).unwrap();

        chat_window(
            &mut stdout,
            &chat,
            Rect {
                x: 0,
                y: 0,
                w: w as usize,
                h: h as usize - 2,
            },
        );

        stdout.queue(MoveTo(0, h - 2)).unwrap();
        stdout.write(bar.as_bytes()).unwrap();

        stdout.queue(MoveTo(0, h - 1)).unwrap();
        stdout.write(prompt.as_bytes()).unwrap();

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(33));
    }

    terminal::disable_raw_mode().unwrap();
}
