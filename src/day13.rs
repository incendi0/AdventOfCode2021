use crate::day13::Instruction::{FoldLeft, FoldUp};
use std::collections::HashSet;

#[derive(Debug)]
pub enum Instruction {
    FoldUp(usize),
    FoldLeft(usize),
}

#[derive(Debug)]
pub struct Paper {
    x: usize,
    y: usize,
    dots: HashSet<(usize, usize)>,
}

impl Paper {
    fn new() -> Self {
        Paper {
            x: 0,
            y: 0,
            dots: HashSet::new(),
        }
    }
}

pub fn first_part(paper: &mut Paper, instructions: &Vec<Instruction>) -> i32 {
    match instructions[0] {
        FoldUp(pivot) => fold_up(paper, pivot),
        FoldLeft(pivot) => fold_left(paper, pivot),
    }
    paper.dots.len() as i32
}

pub fn second_part(paper: &mut Paper, instructions: &Vec<Instruction>) -> i32 {
    for ins in instructions {
        match ins {
            FoldUp(pivot) => fold_up(paper, *pivot),
            FoldLeft(pivot) => fold_left(paper, *pivot),
        }
    }
    let mut xs = vec![vec!["."; paper.x + 1]; paper.y + 1];
    for &(x, y) in paper.dots.iter() {
        xs[y][x] = "#";
    }
    let mut graph = String::new();
    for row in xs {
        for e in row {
            graph.push_str(e);
        }
        graph.push_str("\n");
    }
    paper.dots.len() as i32
}

fn fold_left(paper: &mut Paper, pivot: usize) {
    let mut delta = 0;
    if pivot * 2 < paper.x {
        delta = paper.x - pivot * 2;
        let adapted_dots: HashSet<(usize, usize)> =
            paper.dots.iter().map(|&(x, y)| (x + delta, y)).collect();
        paper.dots = adapted_dots;
    }
    let remain_dots: HashSet<(usize, usize)> = paper
        .dots
        .iter()
        .flat_map(|&(x, y)| {
            if x == pivot {
                None
            } else if x > pivot {
                Some((pivot * 2 - x, y))
            } else {
                Some((x, y))
            }
        })
        .collect();
    paper.dots = remain_dots;
    paper.x = pivot + delta - 1;
}

fn fold_up(paper: &mut Paper, pivot: usize) {
    let mut delta = 0;
    if pivot * 2 < paper.y {
        delta = paper.y - pivot * 2;
        let adapted_dots: HashSet<(usize, usize)> =
            paper.dots.iter().map(|&(x, y)| (x, y + delta)).collect();
        paper.dots = adapted_dots;
    }
    let remain_dots: HashSet<(usize, usize)> = paper
        .dots
        .iter()
        .flat_map(|&(x, y)| {
            if y == pivot {
                None
            } else if y > pivot {
                Some((x, pivot * 2 - y))
            } else {
                Some((x, y))
            }
        })
        .collect();
    paper.dots = remain_dots;
    paper.y = pivot + delta - 1;
}

pub fn read_input(s: &str) -> (Paper, Vec<Instruction>) {
    let mut flag = false;
    let mut paper: Paper = Paper::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            flag = true;
            continue;
        }
        if !flag {
            let xs: Vec<usize> = line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let (x, y) = (xs[0], xs[1]);
            paper.x = paper.x.max(x);
            paper.y = paper.y.max(y);
            paper.dots.insert((x, y));
        } else {
            let xs: Vec<&str> = line.split_ascii_whitespace().collect();
            let xs: Vec<&str> = xs[xs.len() - 1].split('=').collect();
            let ins = match xs.as_slice() {
                ["x", num] => FoldLeft(num.parse::<usize>().unwrap()),
                ["y", num] => FoldUp(num.parse::<usize>().unwrap()),
                _ => unreachable!(),
            };
            instructions.push(ins);
        }
    }
    (paper, instructions)
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::read_input;
    use super::second_part;

    #[test]
    fn first_part_works() {
        let s = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let (mut paper, instructions) = read_input(s);
        assert_eq!(first_part(&mut paper, &instructions), 17);
    }

    #[test]
    fn second_part_works() {
        let s = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let (mut paper, instructions) = read_input(s);
        assert_eq!(second_part(&mut paper, &instructions), 16);
    }
}
