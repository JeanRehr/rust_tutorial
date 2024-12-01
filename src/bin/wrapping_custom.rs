use num::{Bounded, Integer};

fn custom_wrapping_add(a: u8, b: u8) -> u8 {
    const MAX: u8 = u8::MAX;
    let sum = (a as u16) + (b as u16);
    (sum % (MAX as u16 + 1)) as u8 //e.g. sum = to 300, 300 mod (255 + 1) = 44
}

fn gen_custom_wrapping_add<T>(a: T, b: T) -> T
where
    T: Integer + Bounded + Copy,
{
    let max = T::max_value();
    let sum = a + b;
    if sum > max {
        sum - (max + T::one()) //e.g. sum = to 300, 300 - (255 + 1) = 44
    } else {
        sum
    }
}

fn main() {
    let a: u8 = 200;
    let b: u8 = 100;

    // Using the manual custom_wrapping_add function
    let result_manual = custom_wrapping_add(a, b);
    println!("Result (manual): {}", result_manual); // Expected: 44

    let c: u16 = 60000;
    let d: u16 = 10000;

    println!("Result (custom, u16): {}", gen_custom_wrapping_add(c, d));
}
