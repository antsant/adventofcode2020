use std::io;
use std::collections::HashMap;

fn main() {
	let mut group_answers = HashMap::new();
	let mut answer_count = 0;

    for _ in 0..2180 {
    	let mut buffer = String::new();
    	io::stdin()
    		.read_line(&mut buffer)
    		.expect("could not read line!");

		let line = buffer.trim();

		if line == "" {
			answer_count += count_answers(&group_answers);
			group_answers = HashMap::new();
		} else {
			for c in line.chars() {
				group_answers.insert(c, true);
			}
		}
    }

    println!("Answer: {}", answer_count);
}

fn count_answers(group_answers: &HashMap<char, bool>) -> u32 {
	let mut count = 0;
	for (_, _) in group_answers {
		count += 1;
	}
	count
}
