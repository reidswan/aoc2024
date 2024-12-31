use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::util::{Coord, Direction, Grid};

const INPUT: &'static str = include_str!("./input.in");

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

pub fn run() {
    let start = Coord(0, 0);
    let end = Coord(WIDTH as isize - 1, HEIGHT as isize - 1);
    let falling_bytes = parse_input(INPUT);
    let bytes_part1 = &falling_bytes[0..1024];
    let grid = construct_map(bytes_part1);
    let shortest_path = dijkstra(&grid, start, end);
    println!("Part 1: {}", shortest_path);

    let Coord(x, y) = first_blocker(grid, start, end, &falling_bytes[1024..]);
    println!("Part 2: {},{}", x, y);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Corrupted,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| Coord(a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn construct_map(falling_bytes: &[Coord]) -> Grid<Cell> {
    let mut grid = Grid::with_dimensions(WIDTH, HEIGHT);

    for byte in falling_bytes {
        grid.set(*byte, Cell::Corrupted);
    }

    grid
}

#[derive(PartialEq, Eq)]
struct HeapEntry {
    location: Coord,
    dist: usize,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Grid<Cell>, start: Coord, end: Coord) -> usize {
    let mut dist_from_start = HashMap::new();
    dist_from_start.insert(start, 0);
    let mut heap = BinaryHeap::from([HeapEntry {
        location: start,
        dist: 0,
    }]);

    while let Some(HeapEntry { location, dist }) = heap.pop() {
        for dir in Direction::all_directions() {
            let new_dist = dist + 1;
            let neighbor = location.move_(dir);
            if grid.get(neighbor) != Some(&Cell::Empty) {
                continue;
            }

            if let Some(&old_dist) = dist_from_start.get(&neighbor) {
                if old_dist > new_dist {
                    dist_from_start.insert(neighbor, new_dist);
                    heap.push(HeapEntry {
                        location: neighbor,
                        dist: new_dist,
                    })
                }
            } else {
                dist_from_start.insert(neighbor, new_dist);
                heap.push(HeapEntry {
                    location: neighbor,
                    dist: new_dist,
                })
            };
        }
    }

    *dist_from_start.get(&end).expect(&format!(
        "did not find a path from {:?} to {:?}",
        start, end
    ))
}

fn reachable(grid: &Grid<Cell>, start: Coord, end: Coord) -> bool {
    let mut visited = HashSet::new();
    let mut stack = vec![start];
    while let Some(next) = stack.pop() {
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);

        if next == end {
            return true;
        }

        stack.extend(
            Direction::all_directions()
                .into_iter()
                .map(|d| next.move_(d))
                .filter(|loc| grid.get(*loc) == Some(&Cell::Empty)),
        )
    }

    false
}

fn first_blocker(mut grid: Grid<Cell>, start: Coord, end: Coord, raining_bytes: &[Coord]) -> Coord {
    for byte in raining_bytes {
        grid.set(*byte, Cell::Corrupted);
        if !reachable(&grid, start, end) {
            return *byte;
        }
    }

    panic!("Path was never blocked!")
}
