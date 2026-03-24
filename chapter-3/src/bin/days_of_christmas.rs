use std::io::{self, Write};

const DAYS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

const GIFTS : [&str; 12] = [
    "A Partridge in a Pear Tree",
    "Two Turtle Doves",
    "Three French Hens",
    "Four Calling Birds",
    "Five Gold Rings",
    "Six Geese a-Laying",
    "Seven Swans a-Swimming",
    "Eight Maids a-Milking",
    "Nine Ladies Dancing",
    "Ten Lords a-Leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers Drumming"
];

fn main(){
    
    let mut day_count = 1;
    for day in DAYS {
        println!("On the {day} day of Christmas, my true love sent to me:");

        for gift in (0..day_count).rev() {
            print!("{}", GIFTS[gift]);
            
            if day_count > 1 {
                if gift == 1{
                    print!(", and ");
                }
                else if gift > 1 {
                    print!(", ");
                }
                else {
                    print!(".");
                }
            }
            else {
                print!(".");
            }

        }
        io::stdout().flush().unwrap();

        println!("\n");
        day_count += 1;
    }
}