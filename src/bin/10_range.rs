fn main() {
    let range = 1..5;
    let range = 1..=5;

    for num in 1..4 {
        println!("num: {}", num);
    }

    for char in 'a'..='f' {
        println!("char: {}", char);
    }
}