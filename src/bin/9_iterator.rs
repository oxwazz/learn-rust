fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // let mut plus_one = vec![];
    // for number in numbers {
    //     plus_one.push(number + 1);
    // }

    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .collect();
    dbg!(plus_one);

    let new_numbers: Vec<_> = numbers
        .iter()
        .filter(|num| num <= &&3)
        .collect();
    dbg!(new_numbers);

    let find_me: Option<_> = numbers
        .iter()
        .find(|num| num == &&40);
    dbg!(find_me);

    let count = numbers
        .iter()
        .count();
    dbg!(count);

    let last: Option<_> = numbers
        .iter()
        .last();
    dbg!(last);

    let min = numbers
        .iter()
        .min();
    dbg!(min);

    let max = numbers
        .iter()
        .max();
    dbg!(max);

    let take: Vec<_> = numbers
        .iter()
        .take(3)
        .collect();
    dbg!(take);
}