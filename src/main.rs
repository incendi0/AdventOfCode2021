use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() -> Result<(), Box<dyn Error>> {
    {
        let xs = read_to_vec("./input/day01.txt")?;
        let xs = xs.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        println!("day01::partA result: {}", day01::first_part(&xs));
        println!("day01::partB result: {}", day01::second_part(&xs));
    }

    {
        let xs = read_to_vec("./input/day02.txt")?;
        println!("day02::partA result: {}", day02::first_part(&xs));
        println!("day02::partB result: {}", day02::second_part(&xs));
    }

    {
        let xs = read_to_vec("./input/day03.txt")?;
        println!("day03::partA result: {}", day03::first_part(&xs));
        println!("day03::partB result: {}", day03::second_part(&xs));
    }

    {
        let xs = read_to_vec("./input/day03.txt")?;
        println!("day03::partA result: {}", day03::first_part(&xs));
        println!("day03::partB result: {}", day03::second_part(&xs));
    }

    {
        let xs = read_to_vec("./input/day04.txt")?;
        let (seq, xxs) = day04::parse_day04(xs);
        println!("day04::partA result: {}", day04::first_part(&seq, &xxs));
        println!("day04::partB result: {}", day04::second_part(&seq, &xxs));
    }

    {
        let xs = read_to_vec("./input/day05.txt")?;
        println!("day05::partA result: {}", day05::count_overlaps(&xs, true));
        println!("day05::partB result: {}", day05::count_overlaps(&xs, false));
    }

    {
        let xs = read_to_vec("./input/day06.txt")?;
        let xs = xs[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        println!("day06::partA result: {}", day06::simulate(&xs, 80));
        println!("day06::partB result: {}", day06::simulate(&xs, 256));
    }

    {
        let xs = read_to_vec("./input/day07.txt")?;
        let mut xs = xs[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        println!("day07::partA result: {}", day07::first_part(&mut xs));
        println!("day07::partB result: {}", day07::second_part(&mut xs));
    }

    {
        let xs = read_to_vec("./input/day08.txt")?;
        println!("day08::partA result: {}", day08::first_part(&xs));
        // println!("day08::partB result: {}", day08::second_part(&mut xs));
    }

    Ok(())
}

fn read_to_vec(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data = File::open(filename)?;
    let data = BufReader::new(data);
    Ok(data.lines().map(|s| s.unwrap()).collect())
}
