#!/usr/bin/env scriptisto

// scriptisto-begin
// script_src: src/main.rs
// build_cmd: cargo build --release
// target_bin: ./target/release/script
// files:
//  - path: Cargo.toml
//    content: |
//     package = { name = "script", version = "0.1.0", edition = "2018"}
//     [dependencies]
// scriptisto-end

use std::env;
use std::io::{self, BufRead};

fn main() {
    const DBG: bool = false;

    let prefix = env::args().nth(1).unwrap();
    let prefix_chars: Vec<char> = prefix.chars().collect();
    let prefix_len = prefix_chars.len();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    'for0: for lineRes in handle.lines() {
        let mut line = lineRes.unwrap();
        let mut empty_line = true;
        'for1: for (i, char) in line.chars().enumerate() {
            empty_line = false;
            if let Some(&c) = prefix_chars.get(i) {
                if c == char {
                    if DBG { println!("c==char={}", c); }
                    continue 'for1;
                } else {
                    println!("{}", line);
                    continue 'for0;
                }
            } else {
                break 'for1;
            }
        }
        if empty_line {
            println!()
        } else {
            let stripped = line.get(prefix_len..).unwrap();
            println!("{}", stripped);
        }
    }

    // handle.lines().map(|line| line.unwrap());
    // println!("hi {}", "foobar".strip_prefix("foo"))
}
