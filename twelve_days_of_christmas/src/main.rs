const COUNT: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn print_intro(verse: usize) {
    println!("On the {} day of Christmas", COUNT[verse]);
    println!("My true love gave to me")
}

fn sing() {
    for verse in 0..12 {
        print_intro(verse);
        for gift in (0..=verse).rev() {
            if verse != 0 && gift == 0 {
                println!("And {}", GIFTS[gift]);
            } else {
                println!("{}", GIFTS[gift]);
            }
        }
        println!("");
    }
}

fn main() {
    sing()
}
