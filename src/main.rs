use std::fmt::Debug;

use colored::Colorize;
use rand::Rng;
use std::fmt;

///This struct is used to allow the `boxes` to be printed neatly
struct Boxes {
    boxes: [(u8, u8); 100],
}
///Generates 100 numbers representing the boxes
fn generate_boxes() -> Boxes {
    let mut boxes = [(0, 0); 100];

    let mut rng = rand::thread_rng();
    for i in 0..100 {
        boxes[i].0 = rng.gen_range(0..100);
    }
    for i in 0..100 {
        boxes[i].1 = boxes[rng.gen_range(0..100)].0;
    }
    Boxes { boxes }
}

impl Debug for Boxes {
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
        println!("{}", format!("Boxes:\n{:?}", generate_boxes()).bold());
        println!("\x1B[2J\x1B[1;1H");
    }
}
