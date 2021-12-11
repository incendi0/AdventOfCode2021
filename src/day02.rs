pub fn first_part(ss: &[String]) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for s in ss {
        let xs: Vec<&str> = s.split(' ').collect();
        if xs.len() < 2 {
            continue;
        }
        let direction = xs[0].trim();
        let num = xs[1].trim().parse::<i32>().expect("Invalid number");
        match direction {
            "forward" => x += num,
            "up" => y -= num,
            "down" => y += num,
            _ => unreachable!(),
        }
    }
    x * y
}

pub fn second_part(ss: &[String]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for s in ss {
        let xs: Vec<&str> = s.split(' ').collect();
        if xs.len() < 2 {
            continue;
        }
        let direction = xs[0].trim();
        let num = xs[1].trim().parse::<i32>().expect("Invalid number");
        match direction {
            "forward" => {
                horizontal += num;
                depth += aim * num;
            }
            "up" => aim -= num,
            "down" => aim += num,
            _ => unreachable!(),
        }
    }
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::second_part;

    #[test]
    fn first_part_works() {
        let xs = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        assert_eq!(first_part(&xs), 150)
    }

    #[test]
    fn second_part_works() {
        let xs = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        assert_eq!(second_part(&xs), 900)
    }
}
