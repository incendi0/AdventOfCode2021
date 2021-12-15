use std::cmp::Ordering;
use std::collections::BinaryHeap;

// BinaryHeap Doc有个dijkstra的实现，直接拿过来了
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    x: usize,
    y: usize,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn first_part(xs: &Vec<Vec<i32>>) -> i32 {
    let (m, n) = (xs.len(), xs[0].len());
    let mut distance = vec![vec![i32::MAX; n]; m];
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: 0, x: 0, y: 0});
    distance[0][0] = 0;

    while let Some(State {cost, x, y}) = heap.pop() {
        if x == m - 1 && y == n - 1 {
            return cost;
        }
        if cost > distance[x][y] {
            continue;
        }
        for (nx, ny) in neighbours((x, y), m, n) {
            let next_state = State {cost: cost + xs[nx][ny], x: nx, y: ny};
            if next_state.cost < distance[nx][ny] {
                heap.push(next_state);
                distance[nx][ny] = next_state.cost;
            }
        }
    }
    -1
}

fn neighbours(pos: (usize, usize), m: usize, n: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let (x, y) = pos;
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
            ret.push((nx as usize, ny as usize));
        }
    }
    ret
}

pub fn second_part(xs: &Vec<Vec<i32>>) -> i32 {
    let (m, n) = (xs.len(), xs[0].len());
    let mut expanded = vec![vec![0; n * 5]; m * 5];
    for i in 0..m*5 {
        for j in 0..n*5 {
            let (x, y) = (i % m, j % n);
            let v = xs[x][y] + (i / m) as i32 + (j / n) as i32;
            expanded[i][j] = v % 10 + v / 10;
        }
    }
    first_part(&expanded)
}


#[cfg(test)]
mod tests {
    use super::first_part;
    use crate::day09::read_to_matrix;
    use super::second_part;

    #[test]
    fn first_part_works() {
        let s = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let xs = read_to_matrix(s);
        assert_eq!(first_part(&xs), 40);
    }

    #[test]
    fn second_part_works() {
        let s = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let xs = read_to_matrix(s);
        assert_eq!(second_part(&xs), 315);
    }
}
