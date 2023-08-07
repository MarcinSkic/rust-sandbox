struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // implicit deref coercion
    {
        let m = MyBox::new(String::from("Rust"));

        // implicit
        hello(&m);

        // explicit
        hello(&(*m)[..]);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}