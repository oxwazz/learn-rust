fn maybe_string() -> Option<String> {
    Some("hello".to_owned())
}

fn main() {
    let word_length = maybe_string()
        .map(|word| word.len())
        .map(|len| len * 2);

    println!("word length: {:?}", word_length);
}