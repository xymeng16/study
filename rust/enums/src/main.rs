#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // rust enum can store values
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // print something
        println!("{:?}", self);
    }
}

// Now, let's talk about Option<T>
// The definition of Option<T> in the standard library is:
// enum Option<T> {
//     Some(T),
//     None,
// }
// It is such a very useful feature in Rust, hence included
// in prelude. You can use it without Option::

/*
作为一个例子，让我们修改枚举的一个成员来存放数据。1999 年到 2008 年间，
美国在 25 美分的硬币的一侧为 50 个州的每一个都印刷了不同的设计。
其他的硬币都没有这种区分州的设计，所以只有这些 25 美分硬币有特殊的价值。
可以将这些信息加入我们的 enum，通过改变 Quarter 成员来包含一个 State 值，
*/
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // a match block can return any type of value
        // for multi-line code, you could use block
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

/*
我们想要编写一个函数，它获取一个 Option<i32> ，如果其中含有一个值，
将其加一。如果其中没有值，函数应该返回 None 值，而不尝试执行任何操作。
*/
fn plus_one(x: Option<i32>) -> Option<i32> {
    // since x is immutable reference, its ownership would not be borrowed
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
/*
above behavior is common in Rust: match a enum, then bind one
of its value to a variable, and do something with this var...

Note that Rust compiler will automatically check if the match
arm is complete, meaning that you must handle all possible cases
in a match block, otherwise there will be a syntax error, shown as follows:
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
Will cause error:
error[E0004]: non-exhaustive patterns: `None` not covered
 -->
  |
  |         match x {
  |               ^ pattern `None` not covered
*/

// example of the wildcard in match block: _
fn match_wildcard(x: i8) {
    match x {
        1 => println!("one!"),
        2 => println!("two!"),
        3 => println!("three!"),
        _ => println!("otherwise!")
    }
}

// if-let block, a better alternative to match with only one effective case
fn if_let(some_u8_value: Option<u8>) {
    // a bad example
    match some_u8_value {
        Some(3) => println!("threeeeeeeeeee!"),
        _ => println!("None!")
    }
    // a good alternative
    if let Some(3) = some_u8_value {
        println!("threeeeeeeeeee!");
    } else {
        // else is equivalent to the match case _ => ...
        println!("None!");
    }
}



fn main() {
    let message = Message::Move { x: 1, y: 2 };
    message.call();

    let _some_number = Some(1);
    let _some_string = Some(String::from("Hello"));
    let _absent_number: Option<String> = None; // use None need to explicitly indicate the type

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // not work, you cannot add i8 to Option<i8>
    let _sum = x + y.expect("something wrong happened");

    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    let seven = plus_one(six);
    let none = plus_one(None);
    println!(
        "five {:?}, six {:?}, seven {:?}, none {:?}",
        five, six, seven, none
    );

    match_wildcard(121);

    if_let(Some(3u8));
    if_let(None);
}
