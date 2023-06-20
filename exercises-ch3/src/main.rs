use std::{borrow::Borrow, io};

fn main() {
    temperature()
}

fn temperature() {
    println!("Pick your unit [F,C]: ");

    let choice = loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "C" => break "C",
            "F" => break "F",
            &_ => continue,
        }
    };

    let value = loop {};
}

fn from_fahr() {}

fn from_cels() {}
