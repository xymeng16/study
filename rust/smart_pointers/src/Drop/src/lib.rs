#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct MySP {
    data: String,
}

impl Drop for MySP {
    fn drop(&mut self) {
        println!("Dropping MySP with data `{}`!", self.data);
    }
}

pub fn drop_mysp() {
    let c = MySP {
        data: String::from("my stuff"),
    };
    std::mem::drop(c);

    let d = MySP {
        data: String::from("other stuff"),
    };
    println!("MySP created.");
}