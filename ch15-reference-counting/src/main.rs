use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>), // recursive types require some pointer to have finite size
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // basic example
    {
        let _a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&_a));
        let _b = Cons(3, Rc::clone(&_a));
        println!("count after creating b = {}", Rc::strong_count(&_a));
        {
            let _c = Cons(4, Rc::clone(&_a));
            println!("count after creating c = {}", Rc::strong_count(&_a));
        }
        println!(
            "count after c goes out of scope = {}",
            Rc::strong_count(&_a)
        );
    }

    // referenced value is dropped only once, when reference count comes to 0
    {
        struct Example;
        impl Drop for Example {
            fn drop(&mut self) {
                println!("drop");
            }
        }

        let x = Rc::new(Example);
        let y = Rc::clone(&x);
        println!("A");
        drop(x);
        println!("B");
        drop(y);
        println!("C");

        // this will produce: A B drop C
    }
}
