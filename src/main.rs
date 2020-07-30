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
use std::io::{self, Read};
use std::process::exit;

fn to_nul(sep: &mut String) {
    if sep == "\\x00" {
        *sep = String::from("\x00");
        // *sep = *"\x00";
    }
}

fn main() {
    const DBG: bool = false;

    let prefix = env::args().nth(1).unwrap_or(String::from("--help"));
    if &prefix == "-h" || &prefix == "--help" {
        // eprintln!("rmprefix removes the given prefix from its input if present. The input is split using the given separator.\nUsage: rmprefix <prefix> [<separator>=\\n] [<output-separator>=\\n]\nSet the separators to \\x00 for the NUL character.");
        eprintln!(r###"
# Usage

`rmprefix` removes the given prefix from its input if present. The input is split using the given separator, by default newlines.

Usage: `rmprefix <prefix> [<input-separator>=\n] [<output-separator>=\n]`

Set the separators to \x00 for the NUL character.

## Examples

```
echo "The jungles are green.
Some people sing.
The world is not flat.
There are two ways." | rmprefix The
# =>
# jungles are green.
#Some people sing.
# world is not flat.
#re are two ways.

echo "123456789" "23456789" "12345" "Hi" "" "Ocean" | rmprefix 123 ' ' '|||'
# => 456789|||23456789|||45|||Hi||||||Ocean
```

# Installation

```
cargo install --git https://github.com/NightMachinary/rmprefix
```
        "###);
        exit(1)
    }
    let mut sep = env::args().nth(2).unwrap_or(String::from("\n"));
    let mut out_sep = env::args().nth(3).unwrap_or(String::from("\n"));
    to_nul(&mut sep);
    to_nul(&mut out_sep);

    let prefix_chars: Vec<char> = prefix.chars().collect();
    let prefix_len = prefix_chars.len();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut s = String::new();
    handle.read_to_string(&mut s).expect("Couldn't read from stdin.");
    let lines:Vec<&str> = s.split(&sep).collect();
    let lines_len = lines.len();

    'for0: for (ln, line_res) in lines.iter().enumerate() {
        if ln == (lines_len-1) {
            out_sep = "".to_string()
        }
        let line = line_res;
        if line.len() < prefix_len {
            print!("{}{}", line, out_sep);
        }
        else {
            let mut empty_line = true;
            'for1: for (i, char) in line.chars().enumerate() {
                empty_line = false;
                if let Some(&c) = prefix_chars.get(i) {
                    if c == char {
                        if DBG { println!("c==char={}", c); }
                        continue 'for1;
                    } else {
                        print!("{}{}", line, out_sep);
                        continue 'for0;
                    }
                } else {
                    break 'for1;
                }
            }
            if empty_line {
                print!("{}", out_sep)
            } else {
                let stripped = line.get(prefix_len..).unwrap();
                print!("{}{}", stripped, out_sep);
            }
        }

    }

    // handle.lines().map(|line| line.unwrap());
    // println!("hi {}", "foobar".strip_prefix("foo"))
}
