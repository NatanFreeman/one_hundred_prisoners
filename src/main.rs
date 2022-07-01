use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::fmt::Debug;

///This struct is used to allow the `boxes` to be printed neatly
struct Pairs {
    boxes: Vec<(u8, u8)>,
}
///Generates 100 sets of two `u8` with no duplicates
fn generate_boxes() -> Pairs {
    let mut pairs: (Vec<u8>, Vec<u8>) = ((0..100_u8).collect(), (0..100_u8).collect());

    pairs.0.shuffle(&mut thread_rng());
    pairs.1.shuffle(&mut thread_rng());

    let mut res = Vec::new();
    for (index, value) in pairs.0.iter().enumerate() {
        res.push((*value, pairs.1[index]));
    }
    Pairs { boxes: res }
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
        println!("{}", format!("Boxes:\n{:?}", generate_boxes()).bold());
    }
}
