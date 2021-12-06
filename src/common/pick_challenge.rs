use crate::solutions;
use std::error::Error;
use std::fs;

pub fn pick_challenge(day: i32, part: i32, file: String) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(file)?;
    match day {
        1 => match part {
            1 => println!("result = {}", solutions::day1::solve_p1(&data)),
            2 => println!("result = {}", solutions::day1::solve_p2(&data)),
            _ => println!("You chose part {}, but there only 2 parts per day!", part),
        },

        2 => match part {
            1 => println!("result = {}", solutions::day2::solve_p1(&data)),
            2 => println!("result = {}", solutions::day2::solve_p2(&data)),
            _ => println!("You chose part {}, but there only 2 parts per day!", part),
        },

        3 => match part {
            1 => println!("result = {}", solutions::day3::solve_p1(&data)),
            2 => println!("result = {}", solutions::day3::solve_p2(&data)),
            _ => println!("You chose part {}, but there only 2 parts per day!", part),
        },

        4 => match part {
            1 => println!("result = {}", solutions::day4::solve_p1(&data)),
            2 => println!("result = {}", solutions::day4::solve_p2(&data)),
            _ => println!("You chose part {}, but there only 2 parts per day!", part),
        },

        5 => match part {
            1 => println!("result = {}", solutions::day5::solve_p1(&data)),
            2 => println!("result = {}", solutions::day5::solve_p2(&data)),
            _ => println!("You chose part {}, but there only 2 parts per day!", part),
        },

        6 => match part {
            1 => println!("result = {}", solutions::day6::solve_p1(&data)),
            2 => println!("result = {}", solutions::day6::solve_p2(&data)),
            _ => println!("You chose part {}, but there only 2 parts per day!", part),
        },

        _ => println!("Have not finished day {} (yet)...", day),
    };
    Ok(())
}
