use rand::Rng;
use std::collections::HashMap;
fn main() {
    let mut random_integers: [i32; 16] = [0; 16];

    for v in &mut random_integers {
        // this function shouldn't be used when retrieving value within same range more than once
        *v = rand::thread_rng().gen_range(0..8);
    }

    get_median_and_mode(&random_integers);

    let mut sentence = String::from("This is pig latin extreme test");
    convert_to_pig_latin(&mut sentence);
    assert_eq!(sentence, "histay ishay igpay atinlay extremehay esttay");
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

fn convert_to_pig_latin(text: &mut String) {
    let mut converted = String::new();

    for word in text.split_whitespace() {
        let (index, first_char) = word.char_indices().next().unwrap();

        match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => {
                converted = format!("{converted}{word}hay ");
            }
            c => {
                let word = string_without_first_letter_v2(index, c, word);

                converted = format!("{converted}{}{}ay ", word, c.to_ascii_lowercase());
            }
        }
    }

    *text = converted.strip_suffix(" ").unwrap().to_string();
}

#[allow(dead_code)]
fn string_without_first_letter_v1(index: usize, char: char, word: &str) -> &str {
    let next_index = index + char.len_utf8();
    &word[next_index..]
}

#[allow(dead_code)]
fn string_without_first_letter_v2(_index: usize, char: char, word: &str) -> &str {
    &word.strip_prefix(char).unwrap()
}
