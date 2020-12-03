use std::io;

const X_LEN: usize = 31;
const Y_LEN: usize = 323;

// Implement the toboggan trajectory, using concepts from the book thru ch5
// Some code would be cleaner using variable-length collections like Vectors,
// but so far this has not been covered.
fn main() {
    let mut tree_arr: [[bool; X_LEN]; Y_LEN] = [[false; X_LEN]; Y_LEN];
    for i in 0..tree_arr.len() {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error reading input");
        
        for (j, character) in buffer.chars().enumerate() {
            if character == '#' {
                tree_arr[i][j] = true;
            };
        };
    };

    let mut x = 0;
    let mut tree_count = 0;
    for y in 1..tree_arr.len() {
        x += 3;
        if tree_arr[y][x % X_LEN] {
            tree_count += 1;
        };
    };

    println!("Answer: {}", tree_count);
}
