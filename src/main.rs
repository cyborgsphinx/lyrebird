use decrypt::decrypt;

mod decrypt;

fn main() {
    match decrypt("oaxdkyeemhd p") {
        Ok(val) => println!("{}", val),
        Err(..) => println!("Well that went poorly"),
    }
}
