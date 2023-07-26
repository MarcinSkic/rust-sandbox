use std::thread;

fn main() {
    // immutable borrow
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    // mutable borrow
    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);

        // mutable borrow occurs in closure definition, can't borrow immutably here
        //println!("Before calling closure: {:?}", list);
        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    // taking ownership
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // when working will multiple threads data must be moved so that for example data isn't dropped in one thread while it is still used in other thread
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }
}
