use std::io;

const X_LEN: usize = 31;
const Y_LEN: usize = 323;

// Implement the toboggan trajectory, using concepts from the book thru ch5
// Some code would be cleaner using variable-length collections like Vectors,
// but so far this has not been covered.
fn main() {
    let mut tree_arr: [[bool; X_LEN]; Y_LEN] = [[false; X_LEN]; Y_LEN];
    for i in 0..Y_LEN {
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

    let steps = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let mut prod = 1;
    for step in steps.iter() {
        prod *= count_trees(step.0, step.1, tree_arr);
    };

    println!("Answer: {}", prod);
}

fn count_trees(x_step: usize, y_step: usize, tree_arr: [[bool; X_LEN]; Y_LEN]) -> u32 {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut tree_count = 0;
    while y < Y_LEN {
        if tree_arr[y][x % X_LEN] {
            tree_count += 1;
        };
        x += x_step;
        y += y_step;
    };
    tree_count
}
