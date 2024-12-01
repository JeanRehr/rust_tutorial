fn main() {
    /*
    let input_array: [i32; 11] = [1, 2, 0, 23, 14, 65, 0, 21, 34, 0, 2];
    println!("original array: {:?}", input_array);

    let array_of_arrays: Vec<Vec<i32>>;
    array_of_arrays = separate_array_by_null(&input_array);
    println!("separated array into vectors: {:?}", array_of_arrays);

    let sorted_array: Vec<i32>;
    sorted_array = bubble_sort(&input_array);
    println!("sorted array: {:?}", sorted_array);

    println!("original array unmodified: {:?}", input_array);
    */

    let hello: String = String::from("first second third fourth fifth");
    println!("value of the original string: {hello}");

    let word: &str = first_word(&hello);
    println!("word: {word}");

    let two_word: &str = two_words(&hello);
    println!("two word: {two_word}");

    let second_word: &str = only_second_word(&hello);
    println!("only second word: {second_word}");

    let nth_word: &str = get_nth_word(&hello, 2);
    println!("nth word: {nth_word}");
}

fn get_nth_word(s: &str, n: u32) -> &str {
    let bytes: &[u8] = s.as_bytes();
    let mut word_count: u32 = 0;
    let mut count: usize = 0;
    for (i, &value) in bytes.iter().enumerate() {
        if word_count == n {
            return &s[count..i];
        } else if value == b' ' {
            word_count += 1;
        } else if word_count + 1 != n {
            count = i + 2; // space + first letter
        }
    }
    &s[..]
}

fn only_second_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    let mut word_count: u32 = 0;
    let mut count: usize = 0;

    for (i, &value) in bytes.iter().enumerate() {
        if word_count == 2 {
            return &s[count..i];
        } else if value == b' ' {
            word_count += 1;
        } else if word_count == 1 && count == 0 {
            count = i;
        }
    }
    &s[..]
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &value) in bytes.iter().enumerate() {
        if value == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn two_words(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    let mut count: u32 = 0;

    for (i, &value) in bytes.iter().enumerate() {
        if count == 2 {
            return &s[..i];
        } else if value == b' ' {
            count += 1;
        }
    }

    &s[..]
}
/*
fn bubble_sort(slice: &[i32]) -> Vec<i32> {
    let mut sorted_array: Vec<i32> = slice.to_vec();
    for i in 0..sorted_array.len() {
        for j in 0..sorted_array.len() - 1 - i {
            if sorted_array[j] > sorted_array[j + 1] {
                sorted_array.swap(j, j + 1);
            }
        }
    }
    return sorted_array;
}

fn swap_index(arr: &mut [i32], i: usize, j: usize) {
    let temp: i32 = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn separate_array_by_null(slice: &[i32]) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut inside_arrays: Vec<i32> = Vec::new();

    for &i in slice {
        if i == 0 && !inside_arrays.is_empty() {
            matrix.push(inside_arrays);
            inside_arrays = Vec::new();
        } else {
            inside_arrays.push(i);
        }
    }

    if !inside_arrays.is_empty() {
        matrix.push(inside_arrays);
    }

    return matrix;
}
*/
