use std::io;

fn main() {
    let n: u32 = loop {
        println!("What n-th Fibonacci number do you want? n:");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error during input");

        match buffer.trim().parse() {
            Ok(num) => { break num; },
            Err(_) => continue,
        }
    };

    let nth_fib = fib(n);
    println!("The {}-th Fibonacci number: {}", n, nth_fib);
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
