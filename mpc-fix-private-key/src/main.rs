extern crate rand;

use rand::{thread_rng, Rng};

fn generate_random_hex_string(length: usize) -> String {
    let mut rng = thread_rng();
    let hex_chars: Vec<char> = "0123456789abcdef".chars().collect();
    let mut result = String::new();

    for _ in 0..length {
        let idx = rng.gen_range(0..hex_chars.len());
        result.push(hex_chars[idx]);
    }

    result
}

fn split_random_hex_string(hex_string: &str) -> (String, String, String) {
    let first_part = hex_string[0..30].to_string();
    let second_part = hex_string[15..40].to_string();
    let third_part = format!("{}{}", &hex_string[0..15], &hex_string[30..40]);
    (first_part, second_part, third_part)
}


fn recover_string(first_part: &str, second_part: &str, first_part_index: usize, second_part_index: usize) -> String {
    match (first_part_index, second_part_index) {
        (1, 2) => {
            let mut result = first_part.to_string();
            result.push_str(&second_part[15..]);
            result
        },
        (2, 3) => {
            let mut result = second_part[0..15].to_string();
            result.push_str(first_part);
            result
        },
        (1, 3) => {
            let mut result = first_part.to_string();
            result.push_str(&second_part[15..]);
            result
        },
        _ => String::new(),
    }
}



fn main() {
    let hex_string = generate_random_hex_string(40);
    println!("Random hex string: {}", hex_string);

    let (first_part, second_part, third_part) = split_random_hex_string(&hex_string);
    println!("First part: {}", first_part);
    println!("Second part: {}", second_part);
    println!("Third part: {}", third_part);

    println!("");
    let mut recovered_string = recover_string(&first_part, &second_part, 1, 2);

    println!("First part: {}", first_part);
    println!("Second part: {}", second_part);
    println!("Recovered string: {}", recovered_string);

    println!("");

    recovered_string = recover_string(&second_part, &third_part, 2, 3);
    println!("Second part: {}", second_part);
    println!("Third part: {}", third_part);
    println!("Recovered string: {}", recovered_string);

    println!("");

    recovered_string = recover_string(&first_part, &third_part, 1, 3);
    println!("First part: {}", first_part);
    println!("Third part: {}", third_part);
    println!("Recovered string: {}", recovered_string);
}
