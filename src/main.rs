extern crate nix;

mod decrypt;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;
use std::process;

use decrypt::decrypt;

use nix::unistd;

fn main() {
    let mut args = env::args();
    let config = args.nth(1);
    if config.is_some() {
        match begin_decryption(config.unwrap()) {
            Ok(..) => {},
            Err(f) => println!("Error: {}", f),
        };
    }
}

fn begin_decryption(config: String) -> Result<(), String>{
    let mut config_file = match File::open(config).map_err(modify_err) {
        Ok(f) => f,
        Err(e) => return Err(String::from(e))
    };
    let mut st = String::new();
    match config_file.read_to_string(&mut st) {
        Ok(..) => {
            for line in st.lines() {
                let mut files = line.split_whitespace();
                let in_file = files.next().unwrap();
                let out_file = files.next().unwrap();
                match unistd::fork().map_err(modify_err) {
                    Ok(procc) => {
                        if procc.is_child() {
                            match child_work(in_file, out_file) {
                                Ok(..) => {
                                    println!("{} decrypted succesfully", in_file);
                                    process::exit(0);
                                },
                                Err(f) => {
                                    println!("{} failed to decrypt: {}", in_file, f);
                                    process::exit(1);
                                }
                            };
                        } else {
                            println!("Starting work on {}", in_file);
                        }
                    },
                    Err(e) => {
                        return Err(e)
                    }
                };
            }
        },
        Err(..) => println!("Error reading file"),
    }
    Ok(())
}

fn child_work(in_file: &str, out_file: &str) -> Result<usize, String> {
    let mut input = match File::open(in_file).map_err(modify_err) {
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    let mut output = match File::create(out_file).map_err(modify_err) {
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    let mut buf = String::new();
    let _ = match input.read_to_string(&mut buf).map_err(modify_err) {
        Ok(..) => {},
        Err(e) => return Err(e)
    };
    let mut count = 0;

    for line in buf.lines() {
        match decrypt(line) {
            Ok(out) => {
                match write!(output, "{}\n", out).map_err(modify_err) {
                    Ok(..) => count += 1,
                    Err(e) => return Err(e)
                };
            },
            Err(1) => println!("Error in {}: Incorrect Length", in_file),
            Err(2) => println!("Error in {}: Unknown Character", in_file),
            Err(..) => println!("Error in {}: Unknown Error", in_file)
        };
    }

    if count < buf.lines().count() {
        Err(String::from("Some lines not written"))
    } else {
        Ok(count)
    }
}

fn modify_err<E: Error>(e: E) -> String {
    let st = e.description();
    String::from(st)
}
