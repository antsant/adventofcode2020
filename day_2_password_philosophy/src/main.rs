use std::io;

// Implement the Advent of Code day 2 password checker
// https://adventofcode.com/2020/day/2
// This only uses knowledge from the book through ch4, it would have
// probably been a lot cleaner with structures & collections
fn main() {
    let mut valid_pw_count = 0;
    for _ in 0..1000 {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error reading input");
        
        let mut min: i32 = 0;
        let mut max: i32 = 0;
        let mut character: char = ' ';
        let mut password = "";

        for (j, token) in buffer.split_ascii_whitespace().enumerate() {
            if j == 0 {
                let bytes = token.as_bytes();
                for (k, &item) in bytes.iter().enumerate() {
                    if item == b'-' {
                        min = token[..k].parse().expect("did not parse int");
                        max = token[k+1..].parse().expect("did not parse int");
                    }
                }
            } else if j == 1 {
                character = token.chars().nth(0).expect("did not find a character");
            } else {
                password = token;
            }
        }

        let mut char_count = 0;
        for pw_char in password.chars() {
            if pw_char == character {
                char_count += 1;
            }
        }
        if char_count >= min && char_count <= max {
            valid_pw_count += 1;
        }
    };

    println!("Answer: {}", valid_pw_count);
}
