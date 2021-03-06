use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use crate::day09::read_to_matrix;
use crate::day12::read_to_graph;
use crate::day20::read_images;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day20;
mod day21;
mod day22;

fn main() -> Result<(), Box<dyn Error>> {
    {
        let xs = read_to_vec("./input/day01.txt")?;
        let xs: Vec<i32> = xs.iter().map(|s| s.parse::<i32>().unwrap()).collect();
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
        let xs = xs[0]
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        println!("day06::partA result: {}", day06::simulate(&xs, 80));
        println!("day06::partB result: {}", day06::simulate(&xs, 256));
    }

    {
        let xs = read_to_vec("./input/day07.txt")?;
        let mut xs = xs[0]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        println!("day07::partA result: {}", day07::first_part(&mut xs));
        println!("day07::partB result: {}", day07::second_part(&mut xs));
    }

    {
        let xs = read_to_vec("./input/day08.txt")?;
        println!("day08::partA result: {}", day08::first_part(&xs));
        println!("day08::partB result: {}", day08::second_part(&xs));
    }

    {
        let s = read_to_string("./input/day09.txt")?;
        let xs = read_to_matrix(&s);
        println!("day09::partA result: {}", day09::first_part(&xs));
        println!("day09::partB result: {}", day09::second_part(&xs));
    }

    {
        let xs = read_to_vec("./input/day10.txt")?;
        println!("day10::partA result: {}", day10::first_part(&xs));
        println!("day10::partB result: {}", day10::second_part(&xs));
    }

    {
        let s = read_to_string("./input/day11.txt")?;
        let mut xs = read_to_matrix(&s);
        println!("day11::partA result: {}", day11::first_part(&mut xs));
        let mut xs2 = read_to_matrix(&s);
        println!("day11::partB result: {}", day11::second_part(&mut xs2));
    }

    {
        let s = read_to_string("./input/day12.txt")?;
        let graph = read_to_graph(&s);
        println!("day12::partA result: {}", day12::first_part(&graph));
        println!("day12::partB result: {}", day12::second_part(&graph));
    }

    {
        let s = read_to_string("./input/day13.txt")?;
        let (mut paper, instructions) = day13::read_input(&s);
        println!(
            "day13::partA result: {}",
            day13::first_part(&mut paper, &instructions)
        );
        let (mut paper, instructions) = day13::read_input(&s);
        println!(
            "day13::partB result: {}",
            day13::second_part(&mut paper, &instructions)
        );
    }

    {
        let s = read_to_string("./input/day14.txt")?;
        let (mut xs, rules) = day14::read_rules(&s);
        println!(
            "day14::partA result: {}",
            day14::first_part(&mut xs, &rules)
        );
        let (mut xs, rules) = day14::read_rules(&s);
        println!(
            "day14::partB result: {}",
            day14::second_part(&mut xs, &rules)
        );
    }

    {
        let s = read_to_string("./input/day15.txt")?;
        let xs = day09::read_to_matrix(&s);
        println!("day15::partA result: {}", day15::first_part(&xs));
        println!("day15::partB result: {}", day15::second_part(&xs));
    }

    {
        let s = read_to_string("./input/day16.txt")?;
        println!("day16::partA result: {}", day16::first_part(&s.trim()));
        println!("day16::partB result: {}", day16::second_part(&s.trim()));
    }

    {
        let s = read_to_string("./input/day17.txt")?;
        println!("day17::partA result: {}", day17::first_part(&s));
        println!("day17::partB result: {}", day17::second_part(&s));
    }

    {
        let s = read_to_string("./input/day20.txt")?;
        let (algo, image) = day20::read_images(&s);
        println!("day20::partA result: {}", day20::first_part(algo, image));
        let (algo, image) = day20::read_images(&s);
        println!("day20::partB result: {}", day20::second_part(algo, image));
    }
    
    
    {
        println!("day21::partA result: {}", day21::first_part(1, 10));
        println!("day21::partA result: {}", day21::second_part(1, 10));
    }
    
        {
        let s = read_to_string("./input/day22.txt")?;
        let ops = day22::read_to_commands(&s);
        println!("day22::partA result: {}", day22::first_part(&ops));
        println!("day22::partA result: {}", day22::second_part(&ops));
    }

    Ok(())
}

fn read_to_vec(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data = File::open(filename)?;
    let data = BufReader::new(data);
    Ok(data.lines().map(|s| s.unwrap()).collect())
}

fn read_to_string(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
