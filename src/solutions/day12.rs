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

type CaveMap = HashMap<String, Vec<Cave>>;

fn explore(
    map: &CaveMap,
    start: &Cave,
    num_viable_paths: &mut u32,
    small_caves_visited: &mut HashSet<String>,
) {
    if start.size == Size::Small {
        small_caves_visited.insert(start.name.clone());
    }
    for path in map.get(&start.name).unwrap() {
        if path.name == "end" {
            *num_viable_paths += 1;
            return;
        }
        if small_caves_visited.contains(&path.name) {
            continue;
        }
        explore(map, path, num_viable_paths, small_caves_visited);
    }
    if start.size == Size::Small {
        small_caves_visited.remove(&start.name);
    }
}

pub fn solve_p1(data: &str) -> u32 {
    let map = parse_input(data);
    let mut num_viable_paths = 0;
    for key in map.keys() {
        println!("key = {:?}\npaths = {:?}\n", key, map.get(key).unwrap());
    }
    for start_path in map.get("start").unwrap() {
        explore(&map, start_path, &mut num_viable_paths, &mut HashSet::new());
    }
    return num_viable_paths;
}

pub fn solve_p2(_data: &str) -> u64 {
    1
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

    // #[test]
    // fn test_p2() {
    //     let data = r"5483143223
    // 2745854711
    // 5264556173
    // 6141336146
    // 6357385478
    // 4167524645
    // 2176841721
    // 6882881134
    // 4846848554
    // 5283751526
    // ";
    //     assert_eq!(solve_p2(data), 195);
    // }
}
