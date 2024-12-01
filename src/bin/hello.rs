use std::io;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Entry an array index number");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element: i32 = a[index];

    println!(
        "The value of the element in the index {} is {}",
        index, element
    )
}
