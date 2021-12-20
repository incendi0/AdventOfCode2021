pub fn first_part(algo: Vec<u8>, image: Vec<Vec<u8>>) -> i32 {
    let mut v = b'.';
    let first_image = enhance_once(&algo, &image, v);
    v = algo[0];
    let second_image = enhance_once(&algo, &first_image, v);
    second_image.iter().fold(0, |acc, row| acc + row.iter().filter(|e| **e == b'#').count() as i32)
}

pub fn second_part(algo: Vec<u8>, image: Vec<Vec<u8>>) -> i32 {
    let mut v = b'.';
    let mut image = image;
    for _ in 0..50 {
        image = enhance_once(&algo, &image, v);
        v = if v == b'.' { algo[0] } else { algo[511] };
    }
    image.iter().fold(0, |acc, row| acc + row.iter().filter(|e| **e == b'#').count() as i32)
}

fn print_image(image: &Vec<Vec<u8>>) {
    for row in image.iter() {
        println!("{:?}", String::from_utf8_lossy(row));
    }
    println!();
}

fn enhance_once(algo: &Vec<u8>, image: &Vec<Vec<u8>>, v: u8) -> Vec<Vec<u8>> {
    let (m, n) = (image.len(), image[0].len());
    let mut enhanced_image = vec![vec![0u8; n + 2]; m + 2];
    for i in -1..=m as i32 {
        for j in -1..=n as i32 {
            let mut xs = vec![];
            for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)] {
                let ni = i + dx;
                let nj = j + dy;
                if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                    if image[ni as usize][nj as usize] == b'#' {
                        xs.push(1);
                    } else {
                        xs.push(0);
                    }
                } else {
                    xs.push(if v == b'#' { 1 } else { 0 });
                }
            }
            let mut idx = 0;
            for k in 0..9 {
                idx += xs[k] * 2i32.pow((8 - k) as u32);
            }
            enhanced_image[(i + 1) as usize][(j + 1) as usize] = algo[idx as usize];
        }
    }
    enhanced_image
}

pub fn read_images(s: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let mut algo: Vec<u8> = vec![];
    let mut image: Vec<Vec<u8>> = vec![];
    let mut met_blank = false;
    for line in s.lines().into_iter() {
        if line.is_empty() {
            met_blank = true;
        } else {
            if !met_blank {
                algo.extend(line.as_bytes());
            } else {
                image.push(line.as_bytes().to_vec());
            }
        }
    }
    (algo, image)
}


#[cfg(test)]
mod tests {
    use super::first_part;
    use super::second_part;
    use super::read_images;

    #[test]
    fn first_part_works() {
        let s = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let (algo, image) = read_images(s);
        println!("{:?}", first_part(algo, image));
    }

    #[test]
    fn second_part_works() {
        let s = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let (algo, image) = read_images(s);
        println!("{:?}", second_part(algo, image));
    }
}
