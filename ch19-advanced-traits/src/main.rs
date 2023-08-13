fn main() {
    // trait Add is defined: Add<Rhs=Self>, due to default type generic syntax can be skipped.
    // very useful for extending code without breaking existing code,
    // just slap type that was used so far as default
    {
        use std::ops::Add;

        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    // explicit syntax for calling method with the same name
    {
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }

        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
    }

    // fully qualified syntax for calling associated functions
    // definition of fully qualified syntax:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    {
        trait Animal {
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    // traits requiring functionality of other traits (supertraits)
    {
        use std::fmt;

        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        struct Point {
            x: i32,
            y: i32,
        }

        impl OutlinePrint for Point {}

        let point = Point { x: 5, y: 20 };
        point.outline_print();
    }

    // circumventing orphan rule (for implementing trait on type one of them must be local)
    // by using newtype pattern: creating tuple struct as wrapper
    // no runtime performance penalty! wrapper type is elided at compile time
    {
        use std::fmt;
        use std::ops::Deref;

        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "wrapped print: [{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);

        // now we can work with Wrapper like it is &Vec<String>
        impl Deref for Wrapper {
            type Target = Vec<String>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        // example:
        println!("first = {}", w[0])
    }
}
