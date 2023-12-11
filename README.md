- 53:45 부터 보면 됨 

# Crossterm (docs.rs)

- https://docs.rs/crossterm/latest/crossterm/

- KeyEvent
 - https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html

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

# Let's go

- https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/cb1f4e11afe563e2adcc3d5ac109794d5aab7c81#diff-7f93c4e263c4e9ec748f804c7fd04a3b2fde86ffd741fb5516d67e1097bae4c1

-

- type char https://github.com/YoungHaKim7/tsoding_4at_simple_multi_User_Chat/commit/3f35ac10fba94628a7ee08658fd49eba28d5da2c