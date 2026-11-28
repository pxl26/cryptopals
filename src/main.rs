use std::fs::File;

use cryptopals::tools;

use std::io::BufReader;

mod encoding;
fn main() {
    let path = "alice_in_the_wonderland.txt";
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    tools::word_counter::character_count(&mut reader);
}
