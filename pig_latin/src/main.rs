use std::io;
use std::collections::HashMap;

fn main() {
   	let mut buffer = String::new();
    println!("Enter a sentence: ");

   	io::stdin()
        .read_line(&mut buffer)
        .expect("error reading input");

    let mut translated = String::new();
    let vowels: HashMap<_, _> = vec![('a', true), ('e', true), ('i', true), ('o', true), ('u', true)]
    	.into_iter()
    	.collect();
	for word in buffer.trim().split(' ') {
		match word.chars().next() {
			Some(c) => {
				match vowels.get(&c) {
					Some(_) => {
						translated += &format!("{}-hay ", word);
					},
					None => {
						translated += &format!("{}-{}ay ", String::from(&word[1..]), c);
					}
				}
			},
			None => continue,
		}
	}

	println!("{}", translated.trim());
}
