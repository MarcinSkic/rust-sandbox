#![allow(dead_code)]

use std::io;

fn main() {
    //temperature()
    fibonacci(3);
}

fn twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in 0..12 {
        let day_text = days[day];
        println!("{day_text}")
    }
}

fn fibonacci(n: u32) {
    let mut n1 = 0;
    let mut n2 = 1;

    for _ in 1..n {
        n2 += n1;
        n1 = n2 - n1;
    }

    println!("{n2}");
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

    println!("Paste your value: ");

    let value = loop {
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break value;
    };

    let (result, result_unit) = match choice {
        "C" => (value * 9.0 / 5.0 + 32.0, "F"),
        "F" => ((value - 32.0) * 5.0 / 9.0, "C"),
        &_ => panic!("Impossible result"),
    };

    println!("{value}{choice} is equal to {result}{result_unit}")
}
