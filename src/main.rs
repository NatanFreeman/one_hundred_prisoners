use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::fmt::Debug;

mod tests;

///This struct is used to allow the `boxes` to be printed neatly
pub struct Pairs {
    boxes: Vec<(u8, u8)>,
}

impl Pairs {
    pub fn from_tuple(tuple: (Vec<u8>, Vec<u8>)) -> Self {
        let mut res = Vec::new();
        for (index, value) in tuple.0.iter().enumerate() {
            res.push((*value, tuple.1[index]));
        }
        Self { boxes: res }
    }
}
///Generates a couple a `Vec`s with shuffled unique values
pub fn generate_numbers() -> (Vec<u8>, Vec<u8>) {
    let mut pairs: (Vec<u8>, Vec<u8>) = ((0..100_u8).collect(), (0..100_u8).collect());

    pairs.0.shuffle(&mut thread_rng());
    pairs.1.shuffle(&mut thread_rng());
    pairs
}

impl Debug for Pairs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut print = String::new();
        for y in 0..10 {
            for x in 0..10 {
                let pos = y * 10 + x;
                print.push_str(&format!(
                    "{}{} ",
                    format!("{:02}", &self.boxes[pos].0).green(),
                    format!("{:02}", &self.boxes[pos].1).yellow(),
                ));
            }
            print.push('\n');
        }
        f.write_str(&print)
    }
}

fn main() {
    loop {
        println!("\x1B[2J\x1B[1;1H");
        let boxes = Pairs::from_tuple(generate_numbers());
        println!("{}", format!("Boxes:\n{:?}", boxes).bold());
    }
}
