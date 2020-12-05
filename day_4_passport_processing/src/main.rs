use std::io;

#[derive(Debug)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn is_valid(&self) -> bool {
        self.birth_year != None
            && self.issue_year != None
            && self.expiration_year != None
            && self.height != None
            && self.hair_color != None
            && self.eye_color != None
            && self.passport_id != None
    }
}

fn main() {
    let mut valid_passport_count = 0;
    let mut current_passport = Passport::new();
    for _ in 0..1134 {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("error reading input");
        
        if buffer.trim() == "" {
            if current_passport.is_valid() {
                valid_passport_count += 1;
            };
            current_passport = Passport::new();
        } else {
            for field in buffer.split_ascii_whitespace() {
                let parsed = (&field[..3], &field[4..]);
                match parsed {
                    ("byr", val) => current_passport.birth_year = Some(String::from(val)),
                    ("iyr", val) => current_passport.issue_year = Some(String::from(val)),
                    ("eyr", val) => current_passport.expiration_year = Some(String::from(val)),
                    ("hgt", val) => current_passport.height = Some(String::from(val)),
                    ("hcl", val) => current_passport.hair_color = Some(String::from(val)),
                    ("ecl", val) => current_passport.eye_color = Some(String::from(val)),
                    ("pid", val) => current_passport.passport_id = Some(String::from(val)),
                    ("cid", val) => current_passport.country_id = Some(String::from(val)),
                    _ => continue
                };
            };
        };
    };

    println!("Answer: {}", valid_passport_count);
}
