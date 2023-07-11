#![allow(dead_code)]

use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

// generic implementation
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// implemented only for types that implement specified trait
impl<T: Display> Point<T> {
    fn display(&self) {
        println!("Point: {} {}", self.x, self.y);
    }
}

// concrete implementation, only for Point with f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> MixedPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixedPoint<X2, Y2>) -> MixedPoint<X1, Y2> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
