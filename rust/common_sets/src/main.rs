fn main() {
    let mut v: Vec<i32> = Vec::new(); // explicitly indicate the type of Vec<T> in
    let mut vv = vec![1, 2, 3, 4, 5, 6];

    v.push(5);
    v.push(6);
    vv.push(7);
    vv.push(8);
    // when Vec is moved, all of the contents inside it will also be moved.

    let third: &i32 = &vv[2];
    println!("The third element is {}", third);

    if let Some(shit) = vv.get(2) {
        println!("if-let: The third element is {}", shit);
    } else {
        println!("if-not: There is no third element");
    }
    // let does_not_exist = &v[100]; // will cause panic
    let does_not_exist = v.get(100); // will return a None
    match vv.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut vvv = vec![1, 2, 3, 4, 5];

    let first = &vvv[0];

    // vvv.push(6); // not work, since the first val is borrowed before and used later
    // if the pre-allocated space is not enough, rust will re-allocate a new space to vvv and
    // copy the original contents to there, incurring a dangling reference for first

    println!("The first element is {}", first);

    for i in &vvv {
        /*
        must use &vvv instead of vvv since the explicitly call to .into_iter will move vvv out of the current scope
        */
        println!("{}", *i); // you can use *i or i here, both are legal
    }
    println!("The second element is {}", vvv[2]);

    // One may use enum to save multiple type in a Vec
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for e in &row {
        println!("{:?}", e);
    }

    // Now let's talk about String
    let mut s = String::new(); // create a new string
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string(); // can also be invoked via a literal value
    let s = String::from("initial contents");

    // String is encoded as UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // String Update
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    /*
    + operator invokes add function, its signature is like:
    fn add(self, s: &str) -> String {
    that's why the second operand must be a reference,
    also the &String type will be coerced to &str, such is the
    so-called deref coerced technique.
    After the addition, the ownership of s1 is moved since in
    the function signature it is self, but not &self.
    Hence, ithout copy, + operator get the ownership of s1, then push s2
    to the end of s1, and return the ownership of s1. Above zero-copy
    design is more efficient.
    */

    // method to cascade multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // ugly...

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // beautiful

    // Try to index String
    let s = "hello?";
    let ss = String::from("hello?");

    // let ch = s[0];
    // let chch = ss[0];
    // all illegal since &str and String cannot be indexed
    // why? let's discuss below

    /*
    TBH, String is a encapsulation of Vec<u8>:
    pub struct String {
        vec: Vec<u8>,
    }
    And notice that String in Rust is encoded into UTF-8, thus there is no guarantee
    that one byte(u8) is bound to one character... like in Chinese, 你好 has len=4,
    hence with 4 byte and 你 is 2 bytes long.
    So it is really a bad idea to index String
    */

    // In order to traverse String/&str:
    for c in "안녕하세요".chars() {
        println!("{}", c);
    }
    for b in "안녕하세요".bytes() {
        println!("{}", b);
    }

    // what about HashMap?
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /*
    另一个构建哈希 map 的方法是使用一个元组的 vector 的 collect 方法，
    其中每个元组包含一个键值对。collect 方法可以将数据收集进一系列的集合类型，
    包括 HashMap。例如，如果队伍的名字和初始分数分别在两个 vector 中，
    可以使用 zip 方法来创建一个元组的 vector，其中 “Blue” 与 10 是一对，
    依此类推。接着就可以使用 collect 方法将这个元组 vector 转换成一个 HashMap
    */
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // 'zips up' two iterators into a single iterator of pairs, and collect into HashMap

    // let's talk about the ownership related to HashMap
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // now the ownerships of field_* are transferred to HashMap

    // How to access HashMap?
    let val = map.get(&String::from("Favorite color"));
    println!("{:?}", val);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Update HashMap

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrite Blue: 10 as Blue: 25

    println!("{:?}", scores);

    // insert when empty
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);

    // update based on its old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // a mutable reference to the corresponding value
    }

    println!("{:?}", map);
}
