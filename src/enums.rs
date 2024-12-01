enum IpAddr {
    V4(String),
    V6(String),
}

fn route(ip_kind: IpAddr) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/* same as
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

impl Message {
    fn call(&self) {
        /*
        match self {
            Message::Quit => {
                println!("Quit");
            }
            Message::Move { x, y } => {
                println!("Moving to x: {}, y: {}", x, y);
            }
            Message::Write(text) => {
                println!("{}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to r: {}, g: {}, b: {}", r, g, b);
            }
        } */
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //states
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

pub fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in (i + 1)..nums.len() - 1 {
            if nums[i] + nums[j] == target {
                return [i as i32, j as i32].to_vec();
            }
        }
    }
    Vec::new()
}

pub fn sec_higher(vec: Vec<i32>) -> i32 {
    if vec.len() < 2 {
        println!("Array must have a length of at least 2 or higher.");
        return -1;
    }
    let mut higher: i32 = -100;
    let mut sec_higher: i32 = -100;
    for i in 0..vec.len() {
        if vec[i] > higher {
            sec_higher = higher;
            higher = vec[i];
        } else if vec[i] > sec_higher && vec[i] != higher {
            sec_higher = vec[i];
        }
    }
    return sec_higher;
}

fn main() {
    let four: IpAddr = IpAddr::V4("111.0.0.1".to_string());
    let six: IpAddr = IpAddr::V6("::2".to_string());
    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));
    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    route(IpAddr::V4("Test".to_string()));

    //let m: Message = Message::Write(String::from("test"));
    //m.call();

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    let dice_roll: u8 = 9;
    match dice_roll {
        3 => fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other), //catch all (other)
        //_ => rerrol(), //catch all pattern (_)
        _ => (), //do nothing () (empty tuple type)
    }

    //let nums: Vec<i32> = [3, 2, 3].to_vec();
    //let target: i32 = 6;

    //println!("{:?}", two_sums(nums, target));

    let vec_num: Vec<i32> = [3, 5, 32, 6, 31, 18, 3, 2, 1, 7, 16, 3, 10, 32, 33, 31, 11].to_vec();

    println!("{:?}", sec_higher(vec_num));
}

fn fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn rerrol() {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
