use aoc::{Movement, Turtle};

fn main() {
    let filename = std::env::args()
        .skip(1) // Skiping the name of the binary
        .next()
        .expect("give me the path to your program");

    let file = std::fs::read(filename).expect("Can’t open file");
    let input: String = file.iter().cloned().map(|el| el as char).collect();

    let iter = input
        .split(",")
        .map(|el| el.trim())
        .filter_map(|s| s.parse::<Movement>().ok())
        .flat_map(|m| m.explode());

    let mut set = std::collections::HashSet::new();
    let mut bot = Turtle::new();

    for mov in iter {
        if !set.insert(bot.coord) {
            break;
        }
        bot += mov;
    }

    println!("{}", bot.distance_from_base());
}
