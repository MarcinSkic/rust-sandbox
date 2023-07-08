use rand::Rng;
use std::collections::HashMap;
fn main() {
    let mut random_integers: [i32; 16] = [0; 16];

    for v in &mut random_integers {
        // this function shouldn't be used when retrieving value within same range more than once
        *v = rand::thread_rng().gen_range(0..8);
    }

    get_median_and_mode(&random_integers);
}

fn get_median_and_mode(integers: &[i32]) -> (i32, i32) {
    let mut integers = integers.to_vec();
    integers.sort_unstable();
    let median = integers[integers.len() / 2];

    println!("List: {integers:?}");
    println!("Median: {median}");

    let mut occurences = HashMap::new();

    let mut mode = 0;
    let mut max_occurence = 0;

    for v in integers {
        let count = occurences
            .entry(v)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if max_occurence < *count {
            max_occurence = *count;
            mode = v;
        }
    }

    println!("Map: {occurences:?}");
    println!("Mode: {mode} occurs: {max_occurence} times");

    (median, mode)
}
