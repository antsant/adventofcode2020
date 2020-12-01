const LYRICS_PER_DAY: [(&str, &str); 12] = [
    ("first", "a partridge in a pear tree"),
    ("second", "two turtle doves"),
    ("third", "three french hens"),
    ("foruth", "four calling birds"),
    ("fifth", "five golden rings"),
    ("sixth", "six geese a laying"),
    ("seventh", "seven swans a singing"),
    ("eighth", "eight maids a milking"),
    ("ninth", "nine ladies dancing"),
    ("tenth", "ten lords a laying"),
    ("eleventh", "eleven pipers piping"),
    ("twelfth", "twelve drummers drumming")
];

fn main() {
    let mut count = 0;
    for day_lyric in LYRICS_PER_DAY.iter() {
        let day_num_english = day_lyric.0;
        println!("On the {} day of Christmas", day_num_english);
        println!("my true love sent to me");
        
        count += 1;
        for i in (0..count).rev() {
            let lyric = LYRICS_PER_DAY[i].1;
            if count > 1 && i == 0 {
                print!("and ")
            }
            println!("{}", lyric)
        }

        println!();
    }
}
