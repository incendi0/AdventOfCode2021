use std::collections::{HashMap, HashSet};

pub fn first_part(graph: &HashMap<&str, Vec<&str>>) -> i32 {
    let mut ret = 0;
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert("start");
    dfs(graph, &mut ret, "start", &mut visited);
    ret
}

pub fn second_part(graph: &HashMap<&str, Vec<&str>>) -> i32 {
    let mut ret: HashSet<Vec<&str>> = HashSet::new();
    let small_caves: Vec<&&str> = graph
        .keys()
        .filter(|s| s.chars().next().unwrap().is_lowercase())
        .collect();
    for s in small_caves.clone() {
        if *s == "start" || *s == "end" {
            continue;
        }
        let mut visited = HashMap::new();
        for &cave in small_caves.clone() {
            visited.insert(cave, 1);
        }
        visited.insert("start", 0);
        visited.insert("end", 1);
        visited.insert(s, 2);
        let mut path = vec!["start"];
        dfs2(graph, "start", &mut path, &mut visited, &mut ret);
    }
    ret.len() as i32
}

fn dfs2<'a>(
    graph: &HashMap<&str, Vec<&'a str>>,
    u: &str,
    path: &mut Vec<&'a str>,
    visited: &mut HashMap<&'a str, i32>,
    ret: &mut HashSet<Vec<&'a str>>,
) {
    if u == "end" {
        ret.insert(path.clone());
        return;
    }
    if !graph.contains_key(u) {
        return;
    }
    for v in graph.get(u).unwrap() {
        if visited.contains_key(v) && visited.get(v).unwrap() <= &0 {
            continue;
        }
        if v.chars().next().unwrap().is_lowercase() {
            let entry = visited.get_mut(v).unwrap();
            *entry -= 1;
        }
        path.push(v);
        dfs2(graph, v, path, visited, ret);
        path.pop();
        if v.chars().next().unwrap().is_lowercase() {
            let entry = visited.get_mut(v).unwrap();
            *entry += 1;
        }
    }
}

fn dfs<'s>(
    graph: &HashMap<&str, Vec<&'s str>>,
    ret: &mut i32,
    u: &str,
    visited: &mut HashSet<&'s str>,
) {
    if u == "end" {
        *ret += 1;
        return;
    }

    if !graph.contains_key(u) {
        return;
    }

    for v in graph.get(u).unwrap() {
        if visited.contains(v) {
            continue;
        }
        if v.chars().next().unwrap().is_lowercase() {
            visited.insert(v);
        }
        dfs(graph, ret, v, visited);
        if v.chars().next().unwrap().is_lowercase() {
            visited.remove(v);
        }
    }
}

pub fn read_to_graph(s: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    for line in s.split_ascii_whitespace() {
        let xs: Vec<&str> = line.split('-').collect();
        let entry = graph.entry(xs[0]).or_insert(vec![]);
        entry.push(xs[1]);
        let entry_rev = graph.entry(xs[1]).or_insert(vec![]);
        entry_rev.push(xs[0]);
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::first_part;
    use super::read_to_graph;
    use super::second_part;

    #[test]
    fn first_part_works_1() {
        let s = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let graph = read_to_graph(s);
        assert_eq!(first_part(&graph), 10);
    }

    #[test]
    fn first_part_works_2() {
        let s = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let graph = read_to_graph(s);
        assert_eq!(first_part(&graph), 19);
    }

    #[test]
    fn first_part_works_3() {
        let s = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let graph = read_to_graph(s);
        assert_eq!(first_part(&graph), 226);
    }

    #[test]
    fn second_part_works_1() {
        let s = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let graph = read_to_graph(s);
        assert_eq!(second_part(&graph), 36);
    }

    #[test]
    fn second_part_works_2() {
        let s = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let graph = read_to_graph(s);
        assert_eq!(second_part(&graph), 103);
    }

    #[test]
    fn second_part_works_3() {
        let s = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let graph = read_to_graph(s);
        assert_eq!(second_part(&graph), 3509);
    }
}
