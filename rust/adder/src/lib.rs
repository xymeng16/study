#[cfg(test)]
mod tests {
    use super::*;

    #[test] // indicates that this is a tests function
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[tests]
    // fn another(){
    //     panic!("Make this tests fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        let a: i32 = 1;
        assert_eq!(3, add_two(a), "3 is not equal to {}", add_two(a));
    }

    #[test]
    #[should_panic(expected="Guess value must be less than or equal to 100")] // panic message must contains the expected message
    fn greater_than_100() {
        Guess::new(200);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}