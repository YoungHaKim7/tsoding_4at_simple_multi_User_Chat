use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{self, MoveTo},
    event::{
        self, poll, read, Event, KeyCode, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags, KeyEventKind,
    },
    execute,
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

fn main() {
    let mut stdout = stdout();
    // terminal::enable_raw_mode().unwrap();
    let (mut w, mut h) = terminal::size().unwrap();
    let bar_char = "═";
    let mut bar = bar_char.repeat(w as usize);
    let mut prompt = String::new();
    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    bar = bar_char.repeat(w as usize);
                }

                // #[cfg(target_os = "windows")]
                // Event::Key(event) => match event.code {
                //     KeyCode::Char(x) => {
                //         prompt.push(x);
                //         let len_check_prompt = prompt.len();
                //         if len_check_prompt == 2 {
                //             prompt.pop();
                //         } else {
                //             prompt.pop();
                //         }
                //     }
                //     _ => {}
                // },
                Event::Key(event) => match event.code {
                    KeyCode::Char(x) => prompt.push(x),
                    _ => {}
                },
                _ => {}
                KeyEventKind::Release;
            }
        }

        stdout.queue(Clear(ClearType::All)).unwrap();

        stdout.queue(MoveTo(0, h - 2)).unwrap();
        stdout.write(bar.as_bytes()).unwrap();

        stdout.queue(MoveTo(0, h - 1)).unwrap();
        stdout.write(prompt.as_bytes()).unwrap();

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
