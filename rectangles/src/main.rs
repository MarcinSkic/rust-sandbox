#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::square(60);

    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // also correct: Rectangle::area(&rect1)
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    syntactic_sugar_proof();
    consuming_self();
    auto_referencing_sugar();
}

fn syntactic_sugar_proof() {
    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);
}

fn consuming_self() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    println!("{}", rect.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };
    let max_rect = rect.max(other_rect);
    //rect and other_rect doesn't exist anymore at this point;
    println!("{:#?}", max_rect);
}

fn auto_referencing_sugar() {
    // Added the mut keyword to the let-binding
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };

    // same operations
    rect.set_width(1);
    (&mut rect).set_width(1);
    Rectangle::set_width(&mut rect, 1);

    println!("{:#?}", rect);
}
