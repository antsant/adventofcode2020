use std::cmp;
use std::io;

#[derive(Debug)]
enum BinaryControl {
	Higher,
	Lower,
}

fn main() {
    let mut highest_seat_id = 0;
    for _ in 0..761 {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error reading input");

        let mut row_range = (0, 127);
        let mut col_range = (0, 7);
		for (i, character) in buffer.trim().chars().enumerate() {
			if i < 7 {
				if character == 'F' {
					row_range = bin_search(row_range, BinaryControl::Lower);
				} else {
					row_range = bin_search(row_range, BinaryControl::Higher);
				}
			} else {
				if character == 'L' {
					col_range = bin_search(col_range, BinaryControl::Lower);
				} else {
					col_range = bin_search(col_range, BinaryControl::Higher);
				}
			}
		};

		let seat_id = row_range.0 * 8 + col_range.0;
		highest_seat_id = cmp::max(seat_id, highest_seat_id);
	};
    println!("Answer: {}", highest_seat_id);
}

fn bin_search(range: (u32, u32), binary_control: BinaryControl) -> (u32, u32) {
	let step = (range.1 - range.0) / 2 + 1;
	match binary_control {
		BinaryControl::Lower => {
			if step == 1 {
				(range.0, range.0)
			} else {
				(range.0, range.1 - step)
			}
		},
		BinaryControl::Higher => {
			if step == 1 {
				(range.1, range.1)
			} else {
				(range.0 + step, range.1)
			}
		},
	}
}
