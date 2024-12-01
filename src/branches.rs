fn main() {
    let condition: bool = true;

    let number = if condition { 6 } else { 5 };

    println!("number = {number}");
    /*
    let mut count: i32 = 0;
    loop {
        let mut a: String = String::new();
        println!("Your a.");

        io::stdin().read_line(&mut a).expect("failed to read line");

        let a: i32 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut b: String = String::new();
        println!("Your b.");

        io::stdin().read_line(&mut b).expect("failed to read line");

        let b: i32 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("return value of compare: {}", compare(a, b));

        if compare(a, b) == 0 {
            count += 1;
        }

        println!("value of count: {}", count);

        if count == 3 {
            break;
        }
    }
    println!("value of count outside the loop: {}", count);
        */
}

/*
fn compare(a: i32, b: i32) -> i32 {
    return if a > b {
        2
    } else if a < b {
        1
    } else {
        0
    };
}
*/
