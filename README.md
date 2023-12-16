# Crossterm (docs.rs)

- https://docs.rs/crossterm/latest/crossterm/

- KeyEvent
 - https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html

- TcpStream::connect https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html 
  - https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html#method.connect

# tsoding_4at_simple_multi_User_Chat
Simple Multi-User Chat

# Unreasonably Easy Console Apps in Rust | Tsoding Daily
- https://youtu.be/vc5UPu76XOw?si=PotP82sVHQh_X-t2
  - Github
    - https://github.com/tsoding/4at

# Box-Drawing Character

https://en.wikipedia.org/wiki/Box-drawing_character

# Result

```bash
$ cargo run --bin server

Finished dev [unoptimized + debuginfo] target(s) in 0.02s
Running `target/debug/server`
Hello Server

```
# Result

- Client

```
$ cargo run --bin client

   Compiling tsoding_4at_simple_multi_User_Chat v0.1.0 (/home/gy/my_project/rust/111222/tsoding_4at_simple_multi_User_Chat)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/client`
Hello Client

```


# ```cargo tree```

```bash
  

$ cargo tree

tsoding_4at_simple_multi_User_Chat v0.1.0 (/home/gy/my_project/rust/111222/tsoding_4at_simple_multi_User_Chat)
└── crossterm v0.27.0
    ├── bitflags v2.4.1
    ├── libc v0.2.150
    ├── mio v0.8.10
    │   ├── libc v0.2.150
    │   └── log v0.4.20
    ├── parking_lot v0.12.1
    │   ├── lock_api v0.4.11
    │   │   └── scopeguard v1.2.0
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   └── parking_lot_core v0.9.9
    │       ├── cfg-if v1.0.0
    │       ├── libc v0.2.150
    │       └── smallvec v1.11.2
    ├── signal-hook v0.3.17
    │   ├── libc v0.2.150
    │   └── signal-hook-registry v1.4.1
    │       └── libc v0.2.150
    └── signal-hook-mio v0.2.3
        ├── libc v0.2.150
        ├── mio v0.8.10 (*)
        └── signal-hook v0.3.17 (*)

```
# Issues

- WindowsOS 글씨 2번 입력되는거 해결하기 https://github.com/microsoft/terminal/issues/8440
  - https://github.com/crossterm-rs/crossterm/issues/772

- https://github.com/crossterm-rs/crossterm/issues/171 

- OS별 코드 따로 만들기
```rs
#[cfg(target_os = "windows")]
pub const API_KEY: &str = include_str!("..\\..\\key.txt");

#[cfg(target_os = "macos")]
pub const API_KEY: &str = include_str!(r"../../key.txt");

#[cfg(target_os = "linux")]
pub const API_KEY: &str = include_str!(r"../../key.txt");
```

# Let's go

- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/cb1f4e11afe563e2adcc3d5ac109794d5aab7c81#diff-7f93c4e263c4e9ec748f804c7fd04a3b2fde86ffd741fb5516d67e1097bae4c1

- Chat & Print (Client)
 - https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/571a31280de947be62463427f22ac5ebc2a53656 

- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/9a8714ae85c6f25f69382d1aca729e3dac0eb2cc 

- type char https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/3f35ac10fba94628a7ee08658fd49eba28d5da2c

- Usize https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/8d63669d43aa73de2278aec883b89303c9cbe6bd 

- Usize2 https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/2f9d0515773f24af624310355a22e08e79ced245 


# crossterim Tutorial

- Lets build snake with rust https://blog.scottlogic.com/2020/10/08/lets-build-snake-with-rust.html

# crossterm Examples

- https://github.com/crossterm-rs/crossterm/blob/master/examples/event-poll-read.rs

# 방송 순서 
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/cb1f4e11afe563e2adcc3d5ac109794d5aab7c81
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/fa796ddc3ab7d758aac381965b0862f476229f25
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/32990d028f59fe6ee9bc30e5c30599a3d202608a
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/e50201f6fc069cec734ed6830db6facf0531844a
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/5607135f6f2d7ec039b6ffa0d8cf28452e00f8ed
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/ee5c25ebdb5078c9fd9379bfbcf049574dd52a54
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/65b7310df1502cf8429a2e7345a22af71f0dff15
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/18ece555123721620c2c6d867ded140b8bdf3de1
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/6707cff91f0d1b9178c2b3abe3719e788f5f9b91
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/9a8714ae85c6f25f69382d1aca729e3dac0eb2cc
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/8d63669d43aa73de2278aec883b89303c9cbe6bd
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/2f9d0515773f24af624310355a22e08e79ced245
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/632567d693e1ce9f317342d7edd814b8df961ab0
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/dc315a9976373c1f385c7ef09f2a4c2bd6cfa3d0
- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/0971502335afc5bf05d5db9f12dd51e405f4ccf6
