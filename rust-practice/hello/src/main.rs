use std::collections::HashMap;

fn main() {
    // let mut num = 5;
    // num = 3;
    // println!("Hello, world!, {}", num);

    // let arr = [1, 2, 3, 4];
    // for i in arr {
    //     println!("{}", i);
    // }

    // let name = String::from("Bird");
    // let bird = Bird { name, attack: 5 };
    // bird.print_name();

    let mut map = HashMap::new();

    map.insert(0, "Hi");
    map.insert(1, "H12");
    println!("{:?}", map);

    match map.get(&0) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map")
    }

    match map.get(&1) {
        Some(str) => println!("{}", str),
        _ => println!("Doesn't exist in map")
    }

    map.remove(&0);
    println!("{:?}", map)
}

// pub fn is_even(num: u8) -> bool {
//     let digit: u8 = num % 2;
//     digit == 0
// }

// struct Bird {
//     name: String,
//     attack: u64
// }

// impl Bird {
//     fn print_name(&self) {
//         println!("{}", self.name);
//     }
// }
