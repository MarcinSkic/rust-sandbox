#![allow(dead_code)]

fn main() {
    println!("Mutable");
    mutable_reference_to_mutable_value();
    println!("Immutable");
    immutable_reference_to_mutable_value();
    println!("\nMut test");
    mut_test();

    println!("\nSlices");
    slices();

    println!("\nConsuming ownership");
    consuming_ownership();
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

fn round_in_place(v: &mut Vec<f32>) {
    for n in v {
        *n = n.round();
    }
}

fn slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn consuming_ownership() {
    let mut vec: Vec<String> = vec!["Rust".to_string(), "is".to_string(), "awesome!".to_string()];
    vec = add_to_vec(vec, "yello".to_string());
    println!("{:?}", vec)
}

fn add_to_vec(mut vec: Vec<String>, string: String) -> Vec<String> {
    vec.push(string);
    vec
}
