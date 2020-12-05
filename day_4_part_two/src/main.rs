use std::io;

#[derive(Debug)]
struct Passport {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<(u32, String)>,
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

    fn set_birth_year(&mut self, byr: String) {
        if let Ok(val) = byr.parse() {
            if val >= 1920 && val <= 2002 {
                self.birth_year = Some(val);
            };
        }
    }

    fn set_issue_year(&mut self, iyr: String) {
        if let Ok(val) = iyr.parse() {
            if val >= 2010 && val <= 2020 {
                self.issue_year = Some(val);
            }
        }
    }

    fn set_expiration_year(&mut self, eyr: String) {
        if let Ok(val) = eyr.parse() {
            if val >= 2020 && val <= 2030 {
                self.expiration_year = Some(val);
            }
        }
    }

    fn set_height(&mut self, hgt: String) {
        if let Ok(value) = hgt[..hgt.len()-2].parse() {
            let unit = String::from(&hgt[hgt.len()-2..]);
        
            if (unit == "in" && value >= 59 && value <= 76)
                || (unit == "cm" && value >= 150 && value <= 193) {
                self.height = Some((value, unit));
            }    
        }
    }

    fn set_hair_color(&mut self, hcl: String) {
        let mut pass = true;
        for (i, character) in hcl.chars().enumerate() {
            if i == 0 && character != '#' {
                pass = false;
            } else if i > 0
                && !((character >= '0' && character <= '9') || (character >= 'a' && character <= 'f')) {
                pass = false;
            }
        }
        if pass && hcl.len() == 7 {
            self.hair_color = Some(hcl);
        }
    }

    fn set_eye_color(&mut self, ecl: String) {
        let pass = match &ecl[..] {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false
        };
        if pass {
            self.eye_color = Some(ecl);
        }
    }

    fn set_passport_id(&mut self, pid: String) {
        let mut pass = true;
        for (i, character) in pid.chars().enumerate() {
            if !(character >= '0' && character <= '9') {
                pass = false;
            }
        }
        if pass && pid.len() == 9 {
            self.passport_id = Some(pid);
        }
    }

    fn set_country_id(&mut self, cid: String) {
        self.country_id = Some(cid);
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
                println!("{:?}", current_passport);
                valid_passport_count += 1;
            };
            current_passport = Passport::new();
        } else {
            for field in buffer.split_ascii_whitespace() {
                let parsed = (&field[..3], &field[4..]);
                match parsed {
                    ("byr", val) => current_passport.set_birth_year(String::from(val)),
                    ("iyr", val) => current_passport.set_issue_year(String::from(val)),
                    ("eyr", val) => current_passport.set_expiration_year(String::from(val)),
                    ("hgt", val) => current_passport.set_height(String::from(val)),
                    ("hcl", val) => current_passport.set_hair_color(String::from(val)),
                    ("ecl", val) => current_passport.set_eye_color(String::from(val)),
                    ("pid", val) => current_passport.set_passport_id(String::from(val)),
                    ("cid", val) => current_passport.set_country_id(String::from(val)),
                    _ => continue
                };
            };
        };
    };

    println!("Answer: {}", valid_passport_count);
}
