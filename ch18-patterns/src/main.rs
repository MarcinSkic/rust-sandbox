fn main() {
    // match
    {
        let mut x = Some(5);

        x = match x {
            Some(x) => Some(x + 1),
            None => None,
        };

        println!("{x:?}");
    }

    // if let can be mixed and is more flexible than match
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    // while let
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    // for
    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    // let, even it uses patterns
    {
        let _x = 5;
        let (_x, _y, _z) = (1, 2, 3);
    }

    // function parameters
    {
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }

        let point = (3, 5);
        print_coordinates(&point);
    }
}
