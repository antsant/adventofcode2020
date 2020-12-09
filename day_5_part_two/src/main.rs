use std::io;

const INPUT_ROWS: usize = 761;
const SEAT_ID_DIFF: u32 = 2;

#[derive(Debug)]
enum BinaryControl {
	Higher,
	Lower,
}

fn main() {
	let mut seat_ids: [u32; 761] = [0; 761];
    for i in 0..INPUT_ROWS {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error reading input");

        let mut row_range = (0, 127);
        let mut col_range = (0, 7);
		for (j, character) in buffer.trim().chars().enumerate() {
			if j < 7 {
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
		seat_ids[i] = seat_id;
	};

	let mut prev_seat_id = 0;
	seat_ids.sort();
	for seat_id in seat_ids.iter() {
		if seat_id - prev_seat_id == SEAT_ID_DIFF {
			println!("Anser: {}", prev_seat_id + 1);
		}
		prev_seat_id = *seat_id;
	};
}

fn bin_search(range: (u32, u32), binary_control: BinaryControl) -> (u32, u32){
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
