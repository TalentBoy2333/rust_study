pub fn var() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 常量，必须声明类型
    println!("const: {}", THREE_HOURS_IN_SECONDS);

    let x: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("x: [{}, {}, {}]", five_hundred, six_point_four, one);

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 数组
    // let a = [3; 5]; // 等价于 let a = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);

    println!("add func: {}", add(1, 3));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
