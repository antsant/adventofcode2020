use std::io;
use std::collections::HashMap;

struct DepartmentDatabase {
	database: HashMap<String, Vec<String>>,
}

impl DepartmentDatabase {
	fn new() -> DepartmentDatabase {
		DepartmentDatabase {
			database: HashMap::new(),
		}
	}

	fn add_employee(&mut self, employee: &String, department: &String) {
		let department_employees = self.database.entry(String::from(department)).or_insert(vec![]);
		department_employees.push(String::from(employee));
	}

	fn print(&self) {
		for (department, employees) in &self.database {
			println!("{} department employees: {:?}", department, employees);
		}
	}
}

fn main() {
	let mut department_db = DepartmentDatabase::new();

    loop {
    	println!("Enter a command (List, Add [x] to [y], Quit):");
    	let mut buffer = String::new();
	   	io::stdin()
        	.read_line(&mut buffer)
        	.expect("error reading input");
    	
    	let full_command = buffer.trim();
    	let command_type;
 		match full_command.split(' ').next() {
 			Some(word) => command_type = word,
 			None => continue
 		}

 		match command_type {
 			"Add" => {
 				match parse_employee_and_dept(full_command) {
 					Some((employee, department)) => department_db.add_employee(&employee, &department),
 					None => continue
 				}

 			},
 			"List" => department_db.print(),
 			"Quit" => break,
 			_ => continue
 		}
    }
}

fn parse_employee_and_dept(full_command: &str) -> Option<(String, String)> {
	let mut has_parsed_dept = false;
	let mut department = String::new();
	let mut employee = String::new();
	for word in full_command.rsplit(' ') {
		if has_parsed_dept {
			match word {
				"Add" => break,
				_ => employee = format!("{} {}", word, employee),
			};
		} else {
			match word {
				"to" => has_parsed_dept = true,
				_ => department = format!("{} {}", word, department),
			};
		}
	}

	employee = employee.trim().to_string();
	department = department.trim().to_string();
	// If employee is still empty, it implies that either the deparment never finished parsing,
	// or that there was no employee name prior to the expected "to" in the command.
	match &employee[..] {
		"" => None,
		_ => Some((employee, department)),
	}
}
