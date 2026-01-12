use std::fs::read_to_string;

use itertools::Itertools;

type Point = (i32, i32);

// pub fn day9() {
//     let data = read_to_string("puzzles/puz9.txt").expect("A valid fine name is needed");
//     let dk:Vec<Point> = data.lines().map(|x|{
//         let mut it= x.split(",");
//         (it.next().unwrap().parse().unwrap(),it.next().unwrap().parse().unwrap())
//     }).collect();
//     let n = dk.len();
//     let mut edges = Vec::new();
//     for i in 0..n {
//         for j in i+1..n {
//            edges.push((dk[i],dk[j]));
//         }
//     }
//     let mut max_area = 0;
//     for e in edges {
//         max_area =max_area.max(area(e.0,e.1))
//     }
//     println!("{:?}",max_area);

// }

pub fn day9() {
    let data = read_to_string("puzzles/puz9.txt").expect("valid input file");

    // Read red tiles (polygon vertices, in order)
    let points: Vec<Point> = data
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut it = l.split(',');
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    // Polygon edges (circular)
    let polygon_edges: Vec<(&Point, &Point)> = points.iter().circular_tuple_windows().collect();

    // All rectangle candidates from red corner pairs
    let mut rectangles: Vec<((Point, Point), u64)> = Vec::new();
    for (a, b) in points.iter().tuple_combinations() {
        rectangles.push(((*a, *b), area(*a, *b)));
    }

    // Try rectangles from largest area to smallest
    rectangles.sort_by_key(|r| r.1);
    rectangles.reverse();

    let max_area = rectangles
        .iter()
        .find(|((a, b), _)| {
            let rect_left = a.0.min(b.0);
            let rect_right = a.0.max(b.0);
            let rect_bottom = a.1.min(b.1);
            let rect_top = a.1.max(b.1);

            // Rectangle is valid if ALL polygon edges are fully outside
            polygon_edges.iter().all(|(p, q)| {
                let edge_left = p.0.min(q.0);
                let edge_right = p.0.max(q.0);
                let edge_bottom = p.1.min(q.1);
                let edge_top = p.1.max(q.1);

                edge_right <= rect_left
                    || edge_left >= rect_right
                    || edge_top <= rect_bottom
                    || edge_bottom >= rect_top
            })
        })
        .expect("At least one valid rectangle must exist")
        .1;

    println!("{}", max_area);
}

// Inclusive area of rectangle on a tile grid
fn area(a: Point, b: Point) -> u64 {
    let width = (a.0 - b.0).abs() as u64 + 1;
    let height = (a.1 - b.1).abs() as u64 + 1;
    width * height
}
