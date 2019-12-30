#![allow(dead_code)]

use day2;
use day3;
use day5;
use day6;
use day7;

//pub mod day9;

fn main() {
    /*    println!("\n\nDay6:");
    day9::start();*/
    /*    day9::intcode::sum_of_primes();
    day9::intcode::prime_factor_small();
    day9::intcode::prime_factor_big();*/
    println!("\n\nDay2:");
    day2::start(String::from("data/input-02"));
    println!("\n\nDay3:");
    day3::start(String::from("data/input-03"));
    println!("\n\nDay5:");
    day5::start(String::from("data/input-05"));
    println!("\n\nDay6:");
    day6::start(String::from("data/input-06"));
    println!("\n\nDay7:");
    day7::start(String::from("data/input-07"));
    println!("\n\nDay9:");
    day9::start(String::from("data/input-09"));
}
