use rand::Rng;
use std::collections::HashMap;
fn main() {
    // ------------------ EXERCISE 1 -------------------
    let mut random_integers: [i32; 16] = [0; 16];

    for v in &mut random_integers {
        // this function shouldn't be used when retrieving value within same range more than once
        *v = rand::thread_rng().gen_range(0..8);
    }

    get_median_and_mode(&random_integers);

    // ------------------ EXERCISE 2 -------------------
    let mut sentence = String::from("This is pig latin extreme test");
    convert_to_pig_latin(&mut sentence);
    assert_eq!(sentence, "histay ishay igpay atinlay extremehay esttay");

    // ------------------ EXERCISE 3 -------------------
    employees_and_departments();
}

fn get_median_and_mode(integers: &[i32]) -> (i32, i32) {
    let mut integers = integers.to_vec();
    integers.sort_unstable();
    let median = integers[integers.len() / 2];

    println!("List: {integers:?}");
    println!("Median: {median}");

    let mut occurences = HashMap::new();

    let mut mode = 0;
    let mut max_occurence = 0;

    for v in integers {
        let count = occurences
            .entry(v)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if max_occurence < *count {
            max_occurence = *count;
            mode = v;
        }
    }

    println!("Map: {occurences:?}");
    println!("Mode: {mode} occurs: {max_occurence} times");

    (median, mode)
}

fn convert_to_pig_latin(text: &mut String) {
    let mut converted = String::new();

    for word in text.split_whitespace() {
        let (index, first_char) = word.char_indices().next().unwrap();

        match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => {
                converted = format!("{converted}{word}hay ");
            }
            c => {
                let word = string_without_first_letter_v2(index, c, word);

                converted = format!("{converted}{}{}ay ", word, c.to_ascii_lowercase());
            }
        }
    }

    *text = converted.strip_suffix(" ").unwrap().to_string();
}

#[allow(dead_code)]
fn string_without_first_letter_v1(index: usize, char: char, word: &str) -> &str {
    let next_index = index + char.len_utf8();
    &word[next_index..]
}

#[allow(dead_code)]
fn string_without_first_letter_v2(_index: usize, char: char, word: &str) -> &str {
    &word.strip_prefix(char).unwrap()
}

fn employees_and_departments() {
    const EMPLOYEES: [&str; 5] = ["Sarah", "Niles", "Sally", "Amir", "Seth"];
    const DEPARTMENTS: [&str; 3] = ["Engineering", "Sales", "Management"];

    let mut company: HashMap<&str, Vec<&str>> = HashMap::new();

    loop {
        println!(
            "
Options (write number):
1. Add employee to a department
2. Get all people in a department
3. Get all people in the company
4. Quit program"
        );

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Read line failed");

        match choice.trim() {
            "1" => {
                let department = stdin_from_list_of_options(&DEPARTMENTS, "\nPick a department:");
                let employee = stdin_from_list_of_options(&EMPLOYEES, "\nPick a employee:");

                company
                    .entry(&department)
                    .and_modify(|v| v.push(&employee))
                    .or_insert(vec![&employee]);

                println!("Added {employee} to {department}");
            }
            "2" => {
                let department = stdin_from_list_of_options(&DEPARTMENTS, "\nPick a department:");
                match company.get(department) {
                    Some(employees) => println!("Employees: {employees:?}"),
                    None => println!("There are no employees in that department"),
                }
            }
            "3" => println!("Company: {company:?}"),
            "4" => break,
            val => println!("Incorrect value: {val}"),
        }
    }
}

fn stdin_from_list_of_options<'list, T: std::fmt::Display + ?Sized>(
    list: &'list [&T],
    title: &str,
) -> &'list T {
    loop {
        println!("{title}");
        for (index, department) in list.into_iter().enumerate() {
            println!("{}. {}", index + 1, &department);
        }

        let mut choice = String::new();

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Read line failed");

        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Given value is not a number");
                continue;
            }
        };

        match list.get(choice.checked_sub(1).unwrap_or(list.len())) {
            Some(department) => break department,
            None => {
                println!("Choice outside of range");
                continue;
            }
        };
    }
}
