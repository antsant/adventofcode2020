use std::io;
use std::collections::HashMap;

fn main() {
    let mut ints: Vec<i32> = Vec::new();
    let mut int_mode: HashMap<i32, usize> = HashMap::new();

    loop {
    	let mut buffer = String::new();
    	io::stdin()
            .read_line(&mut buffer)
            .expect("error reading input");
        let int_string = buffer.trim();
        if int_string == "" {
        	break;
        };


        let int = int_string.parse().expect("Cannot parse!");
        ints.push(int);

        let count = int_mode.entry(int).or_insert(0);
        *count += 1;
    };

    ints.sort();

    let mut avg: f64 = 0.0;
    for int in &ints {
    	avg += f64::from(*int);
    };
    let n = ints.len();
    avg /= n as f64;

    let median: f64;
    let mid_point = n / 2;
    if n % 2 == 0 {
    	median = (f64::from(ints[mid_point]) + f64::from(ints[mid_point + 1])) / 2.0;
    } else {
    	median = f64::from(ints[mid_point]);
    }

    let mut mode = 0;
    let mut max_count: usize = 0;
    for (int, count) in &int_mode {
    	if *count > max_count {
    		max_count = *count;
    		mode = *int;
    	}
    }

    println!("Average: {}", avg);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
