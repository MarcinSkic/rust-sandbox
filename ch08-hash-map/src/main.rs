use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    /*let stringus = String::from("Blue");
    let stringus2 = String::from("Yellow");

    // map takes ownership of key and value if Copy trait is not present on them
    scores.insert(&stringus, 10);
    scores.insert(&stringus2, 50);*/

    scores.insert(String::from("Blue"), 10);
    // insert can overwrite values
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Yellow")).or_insert(69);
    scores.entry(String::from("Yelo")).or_insert(69);

    let team_name = String::from("Blue");
    // accessing by key
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // iterating through
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("{scores:?}");
}
