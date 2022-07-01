use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;

#[allow(unused_imports)]
use log::{debug, error, info};
use simplelog::*;

mod tests;

///This struct is used to allow the `boxes` to be printed neatly
pub struct Pairs {
    pub numbers: Vec<(u8, u8)>,
}

fn tuple_to_vec(tuple: (Vec<u8>, Vec<u8>)) -> Result<Vec<(u8, u8)>, String> {
    if tuple.0.len() != tuple.1.len() {
        return Err(format!(
            "`Vec`s must have the same length. {} != {}",
            tuple.0.len(),
            tuple.1.len()
        ));
    }
    let mut res = Vec::new();
    for (index, value) in tuple.0.iter().enumerate() {
        res.push((*value, tuple.1[index]));
    }
    Ok(res)
}
impl Pairs {
    pub fn from_tuple(tuple: (Vec<u8>, Vec<u8>)) -> Self {
        Self {
            numbers: tuple_to_vec(tuple).unwrap(),
        }
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
        for _ in 0..10 {
            for _ in 0..10 {
                print.push_str("<green>{:02}</>-<yellow>{0:02}</>");
            }
            print.push('\n');
        }
        f.write_str(&print)
    }
}
pub fn search(prisoner: &(u8, u8), boxes: &Pairs) -> Result<(), String> {
    let mut last_num: u8 = prisoner.0;
    trace!("{:?}", boxes);
    for _ in 0..50 {
        let found = boxes.numbers.iter().find(|x| x.0 == last_num).unwrap();
        debug!(
            "{}{}",
            format!("{:02}", found.0).green(),
            format!("{:02}", found.1).yellow()
        );
        last_num = found.1;
        if last_num == prisoner.0 {
            debug!("Found");
            return Ok(());
        }
    }
    Err("Couldn't find number".to_string())
}
fn simulate() -> Result<(), String> {
    let boxes = Pairs::from_tuple(generate_numbers());
    let mut prisoners: (Vec<u8>, Vec<u8>) = ((0..100_u8).collect(), (0..100_u8).collect());
    prisoners.0.shuffle(&mut thread_rng());
    prisoners.1.shuffle(&mut thread_rng());
    let prisoners = tuple_to_vec(prisoners).unwrap();

    for (index, prisoner) in prisoners.iter().enumerate() {
        let res = search(&prisoner, &boxes);
        if let Ok(_) = res {
            debug!("Prisoner {:02}/100 found his number", index);
            continue;
        } else {
            debug!("Prisoner {:02}/100 couldn't find his number", index);

            return Err(res.unwrap_err());
        }
    }
    Ok(())
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Off,
            Config::default(),
            File::create("main.log").unwrap(),
        ),
    ])
    .unwrap();
    let runs = 10000;
    let mut successes = 0;
    for i in 0..runs {
        info!("simulation: {}", i);
        let res = simulate();
        if res.is_ok() {
            info!("{}", "success".green());
            successes += 1;
        } else {
            info!("{}", "failure".red());
        }
    }
    info!(
        "{}/{} of the simulations were successful ({:.3}%)",
        successes,
        runs,
        successes as f64 / runs as f64 * 100.0
    );
}
