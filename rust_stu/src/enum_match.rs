// 枚举也可以定义关联函数

pub fn new_enum() { 
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home: ");
    dbg!(&home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback: ");
    dbg!(&loopback);
}

pub fn new_enum_2() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("home: ");
    dbg!(&home);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("loopback: ");
    dbg!(&loopback);
}

pub fn new_enum_3() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    println!("home: ");
    dbg!(&home);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("loopback: ");
    dbg!(&loopback);
}


enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// value_in_cents(Coin::Quarter(UsState::Alaska));
pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // TODO: 为什么这里不能跑? 
            // println!("State quarter from {:?}!", state);
            25
        }
    }
}

pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn match_game() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // 通配符，表示其他情况
    }

    fn add_fancy_hat() {
        println!("add fancy hat");
    }
    fn remove_fancy_hat() {
        println!("remove fancy hat");
    }
    fn move_player(num_spaces: u8) {
        println!("move {}", num_spaces); 
    }
}

pub fn match_game_2() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // _ 是一个特殊的模式，可以匹配任意值而不绑定到该值。这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。
    }

    fn add_fancy_hat() {
        println!("add fancy hat");
    }
    fn remove_fancy_hat() {
        println!("remove fancy hat");
    }
    fn reroll() {
        println!("reroll"); 
    }
}

pub fn match_game_3() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // 无事发生
    }

    fn add_fancy_hat() {
        println!("add fancy hat");
    }
    fn remove_fancy_hat() {
        println!("remove fancy hat");
    }
} 

pub fn if_let() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        // println!("State quarter from {}!", state);
        count = 0
    } else {
        count += 1;
    }
    println!("count: {}", count)
} 