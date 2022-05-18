// debug
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

pub fn new_struct() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("active: {}, username: {}, sign: {}", user1.active, user1.username, user1.sign_in_count);
    println!("user1: ");
    dbg!(&user1);

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // 一个简写以上代码的方式
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2: ");
    dbg!(&user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: ");
    dbg!(&black);
    println!("origin: ");
    dbg!(&origin);

    let user3 = build_user(String::from("abc@cba.com"), String::from("abc"));
    println!("user3: ");
    dbg!(&user3);
} 

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn struct_method() {
    // 方法
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // 关联函数
    let sq = Rectangle::square(3);
    println!("square: ");
    dbg!(&sq);
}