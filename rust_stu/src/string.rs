pub fn string_append() -> String{
    // 1
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
    return s; 

    // s2 会覆盖 s1，再次调用 s1 会报错
    /*
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
    */
}

pub fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

// 使用引用可以只用变量的值，而没有所有权，因此在使用后不会释放掉该变量
pub fn calculate_length(s: &String) -> usize { 
    println!("length = {}", s.len());
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn slice() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    println!("first word = {}", word);
    let word = first_word(&my_string[..]);
    println!("first word = {}", word);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);
    println!("first word = {}", word);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    println!("first word = {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("first word = {}", word);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
    println!("first word = {}", word);
}