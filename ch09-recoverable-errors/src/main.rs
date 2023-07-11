use std::fs::File;
use std::io::Read;

fn main() {
    println!("{}", read_username_from_file().unwrap());
}

// return Result type or...
fn read_username_from_file() -> Option<String> {
    //...cast Result to Option with ok()
    let mut username_file = File::open("hello.txt").ok()?;
    let mut username = String::new();
    username_file.read_to_string(&mut username).ok()?;
    Some(username)
}
