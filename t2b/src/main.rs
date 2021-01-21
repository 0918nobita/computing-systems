use std::{
    io::{self, BufRead, Write},
    process,
};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for res in stdin.lock().lines() {
        match res {
            Ok(line) => {
                for hex_str in line.split_ascii_whitespace() {
                    if let Ok(n) = u8::from_str_radix(hex_str, 16) {
                        stdout.write_all(&[n]).unwrap();
                    } else {
                        eprintln!("Failed to convert `{}` to u8", hex_str);
                        process::exit(1);
                    }
                }
            }
            Err(_) => panic!("Failed to read lines from stdin"),
        }
    }
}
