use std::io;

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
