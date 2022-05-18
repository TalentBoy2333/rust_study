use std::io;

mod guess;
mod var;
mod control;
mod string; 
mod structs; 
mod enum_match;

fn main() {
    println!("Hello, world!");

    println!("Please input Part number to study."); 
    println!("Part 1. guess number"); 
    println!("Part 2. variable"); 
    println!("Part 3. controlr"); 
    println!("Part 4. string"); 
    println!("Part 5. struct"); 
    println!("Part 6. enum and match"); 
    println!("==========================");

    let mut part_idx = String::new();
    io::stdin().read_line(&mut part_idx).expect("Fail to read line.");
    let part_idx: u32 = match part_idx.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
    println!("Selected Part {}", part_idx); 
    println!("==========================");

    match part_idx {
        1 => guess::guess_number(), 
        2 => var::var(), 
        3 => {
            control::if_control(); 
            control::if_control_2(); 
            control::loop_control(); 
            control::loop_control_2(); 
            control::while_control(); 
            control::for_control(); 
        }, 
        4 => {
            let s = string::string_append(); 
            string::string_clone(); 
            string::calculate_length(&s); 
            string::slice(); 
        }, 
        5 => {
            structs::new_struct(); 
            structs::struct_method();
        },
        6 => {
            enum_match::new_enum(); 
            enum_match::new_enum_2(); 
            enum_match::new_enum_3();
            let coin = enum_match::Coin::Penny;
            enum_match::value_in_cents(coin); 
            enum_match::plus_one(Option::Some(1)); 
            enum_match::match_game();
            enum_match::match_game_2();
            enum_match::match_game_3();
            enum_match::if_let(); 
        }

        _ => (), 
    }
}