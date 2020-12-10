use std::io;
use std::collections::HashMap;

fn main() {
	let mut group_answers = HashMap::new();
	let mut group_size = 0;
	let mut answer_count = 0;

    for _ in 0..2180 {
    	let mut buffer = String::new();
    	io::stdin()
    		.read_line(&mut buffer)
    		.expect("could not read line!");

		let line = buffer.trim();

		if line == "" {
			answer_count += count_answers(&group_answers, group_size);
			group_answers = HashMap::new();
			group_size = 0;
		} else {
			for c in line.chars() {
				let num_answered = group_answers.entry(c).or_insert(0);
				*num_answered += 1;
			}
			group_size += 1;
		}
    }

    println!("Answer: {}", answer_count);
}

fn count_answers(group_answers: &HashMap<char, u32>, group_size: u32) -> u32 {
	let mut count = 0;
	for (_, num_answered) in group_answers {
		count += if *num_answered == group_size { 1 } else { 0 };
	}
	count
}
