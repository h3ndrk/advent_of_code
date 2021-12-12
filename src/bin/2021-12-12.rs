use std::collections::HashSet;

fn main() {
    let connections = vec![
        ["start", "A"],
        ["start", "b"],
        ["A", "c"],
        ["A", "b"],
        ["b", "d"],
        ["A", "end"],
        ["b", "end"],
    ];

    let paths = traverse(&connections, vec!["start"], None);
    println!("{} Paths: {:?}", paths.len(), paths);
    let mut lowercase_caves = HashSet::new();
    for connection in connections.iter() {
        let is_lowercase = connection[0].to_lowercase() == connection[0];
        if is_lowercase && connection[0] != "start" && connection[0] != "end" {
            lowercase_caves.insert(connection[0]);
        }
        let is_lowercase = connection[1].to_lowercase() == connection[1];
        if is_lowercase && connection[1] != "start" && connection[1] != "end" {
            lowercase_caves.insert(connection[1]);
        }
    }
    let mut paths = HashSet::new();
    for lowercase_cave in lowercase_caves.iter() {
        println!("Allowed Cave: {}", lowercase_cave);
        for path in traverse(&connections, vec!["start"], Some(lowercase_cave)) {
            paths.insert(path);
        }
    }
    println!("{} Paths: {:?}", paths.len(), paths);

    let connections = vec![
        ["dc", "end"],
        ["HN", "start"],
        ["start", "kj"],
        ["dc", "start"],
        ["dc", "HN"],
        ["LN", "dc"],
        ["HN", "end"],
        ["kj", "sa"],
        ["kj", "HN"],
        ["kj", "dc"],
    ];

    let paths = traverse(&connections, vec!["start"], None);
    println!("{} Paths: {:?}", paths.len(), paths);
    let mut lowercase_caves = HashSet::new();
    for connection in connections.iter() {
        let is_lowercase = connection[0].to_lowercase() == connection[0];
        if is_lowercase && connection[0] != "start" && connection[0] != "end" {
            lowercase_caves.insert(connection[0]);
        }
        let is_lowercase = connection[1].to_lowercase() == connection[1];
        if is_lowercase && connection[1] != "start" && connection[1] != "end" {
            lowercase_caves.insert(connection[1]);
        }
    }
    let mut paths = HashSet::new();
    for lowercase_cave in lowercase_caves.iter() {
        println!("Allowed Cave: {}", lowercase_cave);
        for path in traverse(&connections, vec!["start"], Some(lowercase_cave)) {
            paths.insert(path);
        }
    }
    println!("{} Paths: {:?}", paths.len(), paths);

    let connections = vec![
        ["fs", "end"],
        ["he", "DX"],
        ["fs", "he"],
        ["start", "DX"],
        ["pj", "DX"],
        ["end", "zg"],
        ["zg", "sl"],
        ["zg", "pj"],
        ["pj", "he"],
        ["RW", "he"],
        ["fs", "DX"],
        ["pj", "RW"],
        ["zg", "RW"],
        ["start", "pj"],
        ["he", "WI"],
        ["zg", "he"],
        ["pj", "fs"],
        ["start", "RW"],
    ];

    let paths = traverse(&connections, vec!["start"], None);
    println!("{} Paths", paths.len());
    let mut lowercase_caves = HashSet::new();
    for connection in connections.iter() {
        let is_lowercase = connection[0].to_lowercase() == connection[0];
        if is_lowercase && connection[0] != "start" && connection[0] != "end" {
            lowercase_caves.insert(connection[0]);
        }
        let is_lowercase = connection[1].to_lowercase() == connection[1];
        if is_lowercase && connection[1] != "start" && connection[1] != "end" {
            lowercase_caves.insert(connection[1]);
        }
    }
    let mut paths = HashSet::new();
    for lowercase_cave in lowercase_caves.iter() {
        println!("Allowed Cave: {}", lowercase_cave);
        for path in traverse(&connections, vec!["start"], Some(lowercase_cave)) {
            paths.insert(path);
        }
    }
    println!("{} Paths", paths.len());

    let connections = vec![
        ["QR", "da"],
        ["QR", "end"],
        ["QR", "al"],
        ["start", "op"],
        ["zh", "iw"],
        ["zh", "start"],
        ["da", "PF"],
        ["op", "bj"],
        ["iw", "QR"],
        ["end", "HR"],
        ["bj", "PF"],
        ["da", "LY"],
        ["op", "PF"],
        ["bj", "iw"],
        ["end", "da"],
        ["bj", "zh"],
        ["HR", "iw"],
        ["zh", "op"],
        ["zh", "PF"],
        ["HR", "bj"],
        ["start", "PF"],
        ["HR", "da"],
        ["QR", "bj"],
    ];

    let paths = traverse(&connections, vec!["start"], None);
    println!("{} Paths", paths.len());
    let mut lowercase_caves = HashSet::new();
    for connection in connections.iter() {
        let is_lowercase = connection[0].to_lowercase() == connection[0];
        if is_lowercase && connection[0] != "start" && connection[0] != "end" {
            lowercase_caves.insert(connection[0]);
        }
        let is_lowercase = connection[1].to_lowercase() == connection[1];
        if is_lowercase && connection[1] != "start" && connection[1] != "end" {
            lowercase_caves.insert(connection[1]);
        }
    }
    let mut paths = HashSet::new();
    for lowercase_cave in lowercase_caves.iter() {
        println!("Allowed Cave: {}", lowercase_cave);
        for path in traverse(&connections, vec!["start"], Some(lowercase_cave)) {
            paths.insert(path);
        }
    }
    println!("{} Paths", paths.len());
}

fn traverse(
    connections: &Vec<[&'static str; 2]>,
    path: Vec<&'static str>,
    allowed_lowercase: Option<&str>,
) -> Vec<Vec<&'static str>> {
    let neighbors = connections.iter().filter_map(|connection| {
        if connection[0] == *path.last().unwrap() {
            Some(connection[1])
        } else if connection[1] == *path.last().unwrap() {
            Some(connection[0])
        } else {
            None
        }
    });
    // println!("neighbors: {:?}", neighbors);
    let next_visitees = neighbors.filter(|neighbor| {
        let is_allowed_lowercase =
            allowed_lowercase.is_some() && *neighbor == allowed_lowercase.unwrap();
        let is_lowercase = neighbor.to_lowercase() == **neighbor;
        let is_contained_in_path = if is_allowed_lowercase {
            path.iter().filter(|segment| *segment == neighbor).count() > 1
        } else {
            path.contains(neighbor)
        };
        !is_lowercase || (is_lowercase && !is_contained_in_path)
    });
    // println!("next_visitees: {:?}", next_visitees);

    let mut paths = vec![];
    for next_visitee in next_visitees {
        if next_visitee == "end" {
            // println!("Found end: {:?}", path);
            paths.push(path.clone());
            continue;
        }
        // println!("Next Visitee: {}", next_visitee);
        let mut new_path = path.clone();
        new_path.push(next_visitee);
        paths.append(&mut traverse(connections, new_path, allowed_lowercase));
    }

    paths
}
