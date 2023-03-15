enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let maybe_user = Some("oxwazz");
    match maybe_user {
        Some(user) => println!("user: {}", user),
        None => println!("no user")
    }

    if let Some(user) = maybe_user {
        println!("user: {}", user);
    } else {
        println!("no user");
    }


    let red = Color::Red;

    match red {
        Color::Red => println!("red"),
        _ => println!("no user")
    }

    if let Color::Red = red {
        println!("red");
    } else {
        println!("not red");
    }
}