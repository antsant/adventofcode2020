use std::io;

// This program will find three numbers from a list that add to 2020 and output
// their product.
// Quick and dirty implementation using just the basic skills from the book till ch3
// O(n^3) brute-force solution
// Call using stdin:
//     cargo run < input.txt
fn main() {
    let mut input_arr = [0; 200];
    for i in 0..input_arr.len() {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error reading input");
        let next_int: u32 = buffer.trim()
            .parse()
            .expect("input not an number!");
        input_arr[i] = next_int;
    }

    let mut ans = 0;
    for i in 0..input_arr.len() {
        let int1 = input_arr[i];
        for j in i..input_arr.len() {
            let int2 = input_arr[j];
            for k in j..input_arr.len() {
                let int3 = input_arr[k];
                if int1 + int2 + int3 == 2020 {
                    ans = int1 * int2 * int3;
                }
            }
        }
    }
    
    println!("Answer: {}", ans);
}
