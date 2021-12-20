use lazy_static::lazy_static;
use regex::Regex;

pub fn first_part(s: &str) -> i32 {
    let (left_bottom, right_top) = parse_position(s);
    let mut ret = i32::MIN;
    for dx in 1..right_top.x {
        for dy in left_bottom.y..-left_bottom.y {
            ret = ret
                .max(height_with_speed(dx, dy, &left_bottom, &right_top).unwrap_or(left_bottom.y));
        }
    }
    ret
}

pub fn second_part(s: &str) -> i32 {
    let (left_bottom, right_top) = parse_position(s);
    let mut ret = 0;
    for dx in 1..=right_top.x {
        for dy in left_bottom.y..=-left_bottom.y {
            ret += if shoots_to_area(dx, dy, &left_bottom, &right_top) {
                1
            } else {
                0
            };
        }
    }
    ret
}

fn shoots_to_area(dx: i32, dy: i32, left_bottom: &Position, right_top: &Position) -> bool {
    let mut current_pos = Position { x: 0, y: 0 };
    let mut height = 0;
    let (mut dx, mut dy) = (dx, dy);
    let mut flag = false;
    while !judge_below_area(&current_pos, &left_bottom, &right_top) {
        current_pos.x += dx;
        current_pos.y += dy;
        if judge_within_area(&current_pos, &left_bottom, &right_top) {
            flag = true;
            break;
        }
        height = height.max(current_pos.y);
        dx = if dx > 0 { dx - 1 } else { 0 };
        dy = dy - 1;
    }
    flag
}

fn height_with_speed(
    dx: i32,
    dy: i32,
    left_bottom: &Position,
    right_top: &Position,
) -> Option<i32> {
    let mut current_pos = Position { x: 0, y: 0 };
    let mut height = 0;
    let (mut dx, mut dy) = (dx, dy);
    let mut flag = false;
    while !judge_below_area(&current_pos, &left_bottom, &right_top) {
        current_pos.x += dx;
        current_pos.y += dy;
        if judge_within_area(&current_pos, &left_bottom, &right_top) {
            flag = true;
            break;
        }
        height = height.max(current_pos.y);
        dx = if dx > 0 { dx - 1 } else { 0 };
        dy = dy - 1;
    }
    if flag {
        Some(height)
    } else {
        None
    }
}

struct Position {
    x: i32,
    y: i32,
}

fn judge_within_area(pos: &Position, left_bottom: &Position, right_top: &Position) -> bool {
    pos.x >= left_bottom.x && pos.x <= right_top.x && pos.y >= left_bottom.y && pos.y <= right_top.y
}

fn judge_below_area(pos: &Position, left_bottom: &Position, right_top: &Position) -> bool {
    pos.x > right_top.x || pos.y < left_bottom.y
}

fn parse_position(s: &str) -> (Position, Position) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[-]?\d+").unwrap();
    }
    let xs: Vec<i32> = RE
        .find_iter(s)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();
    // println!("parse ret {:?}", xs);
    (
        Position { x: xs[0], y: xs[2] },
        Position { x: xs[1], y: xs[3] },
    )
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::second_part;

    #[test]
    fn first_part_works_1() {
        let s = "target area: x=20..30, y=-10..-5";
        assert_eq!(first_part(s), 45);
    }

    #[test]
    fn second_part_works_1() {
        let s = "target area: x=20..30, y=-10..-5";
        assert_eq!(second_part(s), 112);
    }
}
