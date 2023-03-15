fn main() {
    let add = |x: i32, y: i32| -> i32 {
        x + y
    };

    let subtract = |a, b| a - b;

    println!("add: {:?}", add(1, 2));
    println!("subtract: {:?}", subtract(1, 2));
}