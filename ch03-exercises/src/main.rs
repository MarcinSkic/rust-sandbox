#![allow(dead_code)]

use std::io;

fn main() {
    //temperature()
    twelve_days_of_christmas();
}

fn twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let verses = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 0..12 {
        let day_text = days[day];
        println!("On the {day_text} day of Christmas, my true love sent to me");

        for verse_i in (0..day + 1).rev() {
            let verse_text = verses[verse_i];
            if day != 0 && verse_i == 0 {
                let verse_text = verse_text.to_lowercase();
                println!("And {verse_text}.");
            } else {
                println!("{verse_text}");
            }
        }

        println!();
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
