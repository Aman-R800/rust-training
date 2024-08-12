#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Move { x: 5, y: 10 };
    let msg2 = Message::Write(String::from("Hello, world!"));
    let msg3 = Message::ChangeColor(255, 255, 0);

    match msg1 {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move in the x direction to {} and in the y direction to {}.", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to red {}, green {}, and blue {}.", r, g, b),
    }

    if let Message::Write(text) = msg2 {
        println!("The message is a Write message: {}", text);
    } else {
        println!("The message is not a Write message.");
    }

    let mut stack = vec![Some(3), Some(7), None, Some(5)];
    while let Some(Some(top)) = stack.pop() {
        println!("Stack top: {}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }

    let (x, y, z) = (1, 2, 3);
    println!("Destructured values: x = {}, y = {}, z = {}", x, y, z);

    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    let Point { x: ox, y: oy } = origin;
    println!("The origin is at ({}, {})", ox, oy);

    let mut settings = Some(10);
    let new_setting_value = Some(5);

    match (settings, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing setting."),
        _ => settings = new_setting_value,
    }

    println!("Settings: {:?}", settings);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    let numbers = (1, 2, 3, 4, 5);

    match numbers {
        (first, .., last) => {
            println!("First and last: {} and {}", first, last);
        }
    }

    let num = Some(4);

    match num {
        Some(n) if n < 5 => println!("The number is less than 5: {}", n),
        Some(n) => println!("The number is: {}", n),
        None => (),
    }

    let msg4 = Message::Move { x: 15, y: 20 };

    match msg4 {
        Message::Move { x: x @ 1..=20, y } => println!("Got a Move message with x in range: x = {}, y = {}", x, y),
        Message::Move { x, y } => println!("Got a Move message: x = {}, y = {}", x, y),
        _ => println!("Other message."),
    }
}
