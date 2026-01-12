use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::read_to_string,
};

// pub fn day11() {
//     let data = read_to_string("puzzles/puz11.txt").expect("A valid fine name is needed");
//     let graph: HashMap<&str, Vec<&str>> = data
//         .lines()
//         .map(|x| {
//             let (k, v) = x.split_once(":").unwrap();
//             let v: Vec<&str> = v.split(" ").filter(|x| !x.is_empty()).collect();
//             (k, v)
//         })
//         .collect();
//     let data = traverse_dfs(&graph, "out", "you");
//     println!("{:?}", data.iter().count());
// }

// pub fn traverse_dfs(graph: &HashMap<&str, Vec<&str>>, dest: &str, src: &str) -> Vec<Vec<String>> {
//     let mut visited: HashSet<String> = HashSet::new();
//     let mut result: Vec<Vec<String>> = Vec::new();
//     let mut path: Vec<String> = Vec::new();
//     visited.insert(src.to_string());
//     path.push(src.to_string());
//     dfs(src, graph, &mut visited, dest, &mut result, &mut path);
//     result
// }

// fn dfs(
//     node: &str,
//     graph: &HashMap<&str, Vec<&str>>,
//     visited: &mut HashSet<String>,
//     dest: &str,
//     result: &mut Vec<Vec<String>>,
//     path: &mut Vec<String>,
// ) {
//     if node == dest {
//         result.push(path.clone());
//         return;
//     }
//     match graph.get(node) {
//         Some(neighbours) => {
//             for &n in neighbours {
//                 if visited.contains(n) {
//                     continue;
//                 }
//                 visited.insert(n.to_string());
//                 path.push(n.to_string());
//                 println!("{:?}", n);
//                 dfs(n, graph, visited, dest, result, path);
//                 path.pop();
//                 visited.remove(n);
//             }
//         }
//         None => {
//             return;
//         }
//     }
// }

pub fn day11() {
    let data = read_to_string("puzzles/puz11.txt").expect("A valid fine name is needed");
    let mut graph: BTreeMap<&str, Vec<&str>> = data
        .lines()
        .map(|x| {
            let (k, v) = x.split_once(":").unwrap();
            let v: Vec<&str> = v.split(" ").filter(|x| !x.is_empty()).collect();
            (k, v)
        })
        .collect();
    graph.insert("out", vec![]);
}
