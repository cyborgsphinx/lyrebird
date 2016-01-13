use std::env;
use std::fs::File;
use std::io::{Read, Write, Error};
use decrypt::decrypt;

mod decrypt;

fn main() {
    let mut args = env::args();
    let input = args.nth(1);
    let output = args.next();
    if input.is_some() && output.is_some() {
        match begin_decryption(input.unwrap(), output.unwrap()) {
            Ok(..) => {},
            Err(f) => println!("Error: {}", f),
        };
    }
}

fn begin_decryption(input: String, output: String) -> Result<(), Error>{
    let mut input_file = try!(File::open(input));
    let mut output_file = try!(File::create(output));
    let mut st = String::new();
    match input_file.read_to_string(&mut st) {
        Ok(..) => {
            for line in st.lines() {
                match decrypt(line) {
                    Ok(out) => try!(write!(output_file, "{}\n", out)),
                    Err(1) => println!("Error: Incorrect length"),
                    Err(2) => println!("Error: Unknown character"),
                    Err(..) => println!("Error: Unknown error"),
                }
            }
        },
        Err(..) => println!("Error reading file"),
    }
    Ok(())
}
