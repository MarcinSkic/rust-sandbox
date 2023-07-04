#![allow(dead_code, unused_variables)]
fn main() {
    println!("Hello, world!");
    holding_different_types_of_data();
}

fn creating_vector() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // here type is inferred based on later push operations
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn accesing_elements() {
    let v = vec![1, 2, 3, 4, 5];

    // will panic if there is no element at that index
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // you control how program behaves if there is no element at that index
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn iterating() {
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    // or to edit

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
    }
}

fn holding_different_types_of_data() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // this way you can hold different types in one vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for elem in &row {
        match elem {
            SpreadsheetCell::Int(v) => println!("It is int: {v}!"),
            SpreadsheetCell::Text(v) => println!("It is text: {v}!"),
            SpreadsheetCell::Float(v) => println!("It is float: {v}"),
        }
    }
}

fn crazy_example_that_compiles() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v {
        v2.push(i);
    }
    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];
    println!("{a} {b}");
}
