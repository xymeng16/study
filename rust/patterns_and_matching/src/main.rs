fn main() {
    // conditional if let expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    //while let conditional loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() { // when pop() returns None, the loop stops
        println!("{}", top);
    }

    // for loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let point = (3, 5);
    println!("point({}, {})", point.0, point.1);

    // matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching ranges of values with ..=
    let x = 1;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a' ..='j' => println!("early ASCII letter"),
        'k' ..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructuring to break apart values

    // destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point {x: 0, y: 7};

    let Point {x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axix: ({}, {})", x, y),
    }

    // destructuring enums
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move {x, y} => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    // destructuring nested structs and enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hsv {}, {}, {}",
            h, s, v
        ),
        Message2::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to rgb {}, {}, {}",
            r, g, b
        ),
        _ => (),
    }

    // use only parts of the struct and ignore remaining parts
    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3 { x: 1, y: 0, z: 0};

    match origin {
        Point3 {x, ..} if x == 0 => println!("Point3: x = {}", x), // if x == 0 is the match guard
        _ => {}
    }

    // @ bindings: create a variable that holds a values at the same time we're testing that value to see
    // whether it matches a pattern
    enum Message3 {
        Hello {id: i32},
    }

    let msg = Message3::Hello {id: 5};

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello {id: 10..=12} => {
            println!("Found an id in another range")
        }
        Message3::Hello {id} => println!("Found some other id: {}", id),
    }
}
