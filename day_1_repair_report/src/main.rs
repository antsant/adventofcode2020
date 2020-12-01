use std::io;

// This program will find two numbers from a list that add to 2020 and return
// their product.
fn main() {
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                
            }
            Err(_) => break,
        }
            
    }
}
