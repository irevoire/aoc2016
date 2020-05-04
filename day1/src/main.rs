use day1::{Bot, Movement};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("give me the path to your program"); // Skiping the name of the binary

    let file = std::fs::read(filename).expect("Can’t open file");
    let input: String = file.iter().cloned().map(|el| el as char).collect();

    let mut bot = Bot::new();

    input
        .split(",")
        .map(|el| el.trim())
        .filter_map(|s| s.parse::<Movement>().ok())
        .for_each(|movement| bot += movement);

    println!("{}", bot.distance_from_base());
}
