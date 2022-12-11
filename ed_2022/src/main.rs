use clap::Parser;
use ed_2022::{day1, day2, day3, day4, day5, day6};
use std::fs;

/// Advent of code 2022
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// AOC day to run
    #[arg(short, long)]
    day: u8,

    /// Part of the problem
    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => {
            let file_path = "./data/day1_input.txt";
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");

            match args.part {
                1 => {
                    println!("{}", day1::max_calories(contents))
                }
                2 => {
                    println!("{}", day1::top_three_calories(contents))
                }
                _ => println!("There is no part {}", args.part),
            }
        }
        2 => {
            let file_path = "./data/day2_input.txt";
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");

            match args.part {
                1 => {
                    println!("{}", day2::total_score_own_strat(contents))
                }
                2 => {
                    println!("{}", day2::total_score_elf_strat(contents))
                }
                _ => println!("There is no part {}", args.part),
            }
        }
        3 => {
            let file_path = "./data/day3_input.txt";
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");

            match args.part {
                1 => {
                    println!("{}", day3::sum_misplaced_prio(contents))
                }
                2 => {
                    println!("{}", day3::sum_elf_groups_prio(contents))
                }
                _ => println!("There is no part {}", args.part),
            }
        }
        4 => {
            let file_path = "./data/day4_input.txt";
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");

            match args.part {
                1 => {
                    println!("{}", day4::fully_covered_count(contents))
                }
                2 => {
                    println!("{}", day4::overlaping_count(contents))
                }
                _ => println!("There is no part {}", args.part),
            }
        }
        5 => {
            let file_path = "./data/day5_input.txt";
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");

            match args.part {
                1 => {
                    println!("{}", day5::get_top_crates(contents))
                }
                2 => {
                    println!("{}", day5::get_top_crates_9001(contents))
                }
                _ => println!("There is no part {}", args.part),
            }
        }
        6 => {
            let file_path = "./data/day6_input.txt";
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");

            match args.part {
                1 => {
                    println!("{}", day6::get_start_packet_marker(contents))
                }
                2 => {
                    println!("{}", day6::get_start_message_marker(contents))
                }
                _ => println!("There is no part {}", args.part),
            }
        }
        _ => println!("There are no problems for that day"),
    }
}
