use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Size {
    Small,
    Large,
}

fn from_string(s: &str) -> Size {
    match s == s.to_uppercase() {
        true => Size::Large,
        false => Size::Small,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Cave {
    name: String,
    size: Size,
}

impl Cave {
    fn new(name: &str) -> Self {
        Cave {
            name: name.to_string().clone(),
            size: from_string(name),
        }
    }
}

type CaveMap = HashMap<String, Vec<Cave>>;

pub fn solve_p1(data: &str) -> usize {
    let map = parse_input(data);
    let mut paths: HashSet<Vec<Cave>> = HashSet::new();
    let start_cave = Cave::new("start");
    explore1(&map, &start_cave, &mut paths, &mut vec![start_cave.clone()]);
    return paths.len();
}

pub fn solve_p2(data: &str) -> usize {
    let map = parse_input(data);
    let mut paths: HashSet<Vec<Cave>> = HashSet::new();
    let start_cave = Cave::new("start");
    explore2(&map, &start_cave, &mut paths, &mut vec![start_cave.clone()]);
    return paths.len();
}

fn parse_input(data: &str) -> CaveMap {
    data.lines().fold(HashMap::new(), |mut map, line| {
        let path_tuple = line.split_once('-').unwrap();
        let name1 = path_tuple.0.to_string();
        let name2 = path_tuple.1.to_string();
        let cave1 = Cave {
            name: name1.clone(),
            size: from_string(&name1),
        };
        let cave2 = Cave {
            name: name2.clone(),
            size: from_string(&name2),
        };
        insert_caves(&mut map, name1, cave2.clone());
        insert_caves(&mut map, name2, cave1.clone());
        return map;
    })
}

fn insert_caves(map: &mut CaveMap, cave_start_name: String, cave_end: Cave) {
    if map.contains_key(&cave_start_name) {
        map.get_mut(&cave_start_name).unwrap().push(cave_end);
    } else {
        map.insert(cave_start_name, vec![cave_end]);
    }
}

fn explore1(
    map: &CaveMap,
    start: &Cave,
    paths: &mut HashSet<Vec<Cave>>,
    current_path: &mut Vec<Cave>,
) {
    for path in map.get(&start.name).unwrap() {
        if path.name == "end" {
            paths.insert([current_path.clone(), vec![Cave::new("end")]].concat());
            continue;
        }
        if path.size == Size::Small && current_path.contains(&path) {
            continue;
        }
        current_path.push(path.clone());
        explore1(map, path, paths, current_path);
        current_path.pop();
    }
    return;
}

fn explore2(
    map: &CaveMap,
    start: &Cave,
    paths: &mut HashSet<Vec<Cave>>,
    current_path: &mut Vec<Cave>,
) {
    for path in map.get(&start.name).unwrap() {
        if path.name == "end" {
            paths.insert([current_path.clone(), vec![Cave::new("end")]].concat());
            continue;
        }
        if need_to_skip_this_cave(&path, &current_path) {
            continue;
        }
        current_path.push(path.clone());
        explore2(map, path, paths, current_path);
        current_path.pop();
    }
    return;
}

fn need_to_skip_this_cave(cave: &Cave, path: &Vec<Cave>) -> bool {
    // you can probably figure out how to do this in one expression...
    // large caves are never skipped
    if cave.name == cave.name.to_uppercase() {
        return false;
    }
    // early out when this cave has not been visited yet
    if !path.contains(&cave) {
        return false;
    }

    // "start" and "end" still can only occur once
    if cave.name == "start".to_string() || cave.name == "end".to_string() {
        return true;
    }

    // check if already has a duplicate cave
    if has_small_duplicate(&path) {
        return true;
    }
    return false;
}

fn has_small_duplicate(path: &Vec<Cave>) -> bool {
    for cave in path.iter() {
        if path
            .iter()
            .filter(|y| y.name == y.name.to_lowercase())
            .filter(|x| x.name == cave.name)
            .count()
            >= 2
        {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_1() {
        let data = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
        assert_eq!(solve_p1(data), 10);
    }

    #[test]
    fn test_p1_2() {
        let data = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
        assert_eq!(solve_p1(data), 19);
    }

    #[test]
    fn test_p1_3() {
        let data = r"fs-end
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
start-RW
";
        assert_eq!(solve_p1(data), 226);
    }

    #[test]
    fn test_p2_1() {
        let data = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
        assert_eq!(solve_p2(data), 36);
    }

    #[test]
    fn test_p2_2() {
        let data = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
        assert_eq!(solve_p2(data), 103);
    }

    #[test]
    fn test_p2_3() {
        let data = r"fs-end
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
start-RW
";
        assert_eq!(solve_p2(data), 3509);
    }
}
