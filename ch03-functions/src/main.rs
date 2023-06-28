fn main() {
    println!("Hello, world!");

    another_function(5);
    let x = five();

    println!("The value of x is: {x}");
}

fn another_function(lol: u8) {
    println!("Another function. {lol}");
}

fn five() -> i32 {
    5
}
