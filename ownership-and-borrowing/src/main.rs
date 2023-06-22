fn main() {
    println!("Mutable");
    mutable_reference_to_mutable_value();
    println!("Immutable");
    immutable_reference_to_mutable_value();
    println!("\nMut test");
    mut_test();
}

fn immutable_reference_to_mutable_value() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);
}

fn mutable_reference_to_mutable_value() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let mut vec2 = vec![5, 6, 7];
    let mut num: &mut i32 = &mut vec[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);

    num = &mut vec2[0];

    println!("Fist element is {}", *num);
    println!("Vector2 is now {:?}", vec2);
}

fn mut_test() {
    let mut x = 1;
    let y = &x;
    let mut z = *y;
    x += z;
    z += 5;
    println!("{x}");
    println!("{z}");
}
