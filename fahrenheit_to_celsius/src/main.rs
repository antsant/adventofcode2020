use std::io;

fn main() {
    let temperature: f64 = loop {
        println!("Input a temperature in Fahrenheit:");

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error while reading input");
    
        match buffer.trim().parse() {
            Ok(num) => { break num; },
            Err(_) => continue,
        };
    };

    let temperature = (temperature - 32.0) * (5.0 / 9.0);
    println!("The temperature in Celsius: {}", temperature);
}
