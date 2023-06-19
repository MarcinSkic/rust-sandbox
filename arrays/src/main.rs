use std::io;
fn main() {
    let _a = [1, 2, 3, 4, 5];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _a = [3; 5]; //Array with 5 elements equal 3

    user_index();
}

// Rust will panick when out of bonds, it won't access some random memory
fn user_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
