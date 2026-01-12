use std::{collections::HashMap, fs::read_to_string};

type Point = (i64, i64, i64);

#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
    dist: i64,
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
        return true;
    }
}

fn dist(a: Point, b: Point) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

pub fn day8() {
    let input = read_to_string("puzzles/puz8.txt").unwrap();
    // -------- Parse points --------
    let points: Vec<Point> = input
        .lines()
        .map(|l| {
            let mut it = l.split(',');
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let n = points.len();
    println!("n = {}", n);

    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                u: i,
                v: j,
                dist: dist(points[i], points[j]),
            });
        }
    }

    edges.sort_unstable_by_key(|e| e.dist);

    let mut dsu = Dsu::new(n);
    // let edge_limit = 1000;

    // for e in edges.iter().take(edge_limit) {
    //     dsu.union(e.u, e.v);
    // }
    let mut successful_unions = 0;
    let mut last_edge = None;
    for e in &edges {
        if dsu.union(e.u, e.v) {
            successful_unions += 1;
            last_edge = Some((e.u, e.v));

            if successful_unions == n - 1 {
                break;
            }
        }
    }
    let (a, b) = last_edge.unwrap();
    let ans = points[a].0 * points[b].0;
    println!("{ans}");

    // -------- Count circuits --------
    let mut components: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = dsu.find(i);
        *components.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = components.values().cloned().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    println!("sizes = {:?}", sizes);

    let answer = sizes[0] * sizes[1] * sizes[2];
    println!("answer = {}", answer);
}
