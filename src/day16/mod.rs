use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::util::{Coord, Direction, Grid};

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let maze = parse_input(INPUT);
    let (min_cost, points) = dijkstra(&maze);

    println!("Part 1: {}", min_cost);
    println!("Part 2: {}", points);
}

struct Maze {
    grid: Grid<Cell>,
    start: Coord,
    end: Coord,
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

fn parse_input(input: &str) -> Maze {
    let mut start = None;
    let mut end = None;

    let grid = Grid(
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Cell::Wall,
                        '.' => Cell::Empty,
                        'S' => {
                            start = Some(Coord(x as isize, y as isize));
                            Cell::Empty
                        }
                        'E' => {
                            end = Some(Coord(x as isize, y as isize));
                            Cell::Empty
                        }
                        _ => unreachable!("{}", c),
                    })
                    .collect()
            })
            .collect(),
    );

    Maze {
        grid,
        start: start.expect("did not find start tile"),
        end: end.expect("did not find end tile"),
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct HeapEntry {
    dist: usize,
    location: Coord,
    facing: Direction,
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

fn dijkstra(maze: &Maze) -> (usize, usize) {
    let mut dist_from_start = HashMap::new();
    dist_from_start.insert((maze.start, Direction::Right), 0);
    let mut heap = BinaryHeap::new();
    heap.extend([HeapEntry {
        dist: 0,
        facing: Direction::Right,
        location: maze.start,
    }]);

    while let Some(HeapEntry {
        location,
        facing,
        dist,
    }) = heap.pop()
    {
        for dir in Direction::all_directions() {
            let new_dist = if dir == facing {
                dist + 1
            } else if dir == facing.opposite() {
                continue;
            } else {
                dist + 1001
            };

            let neighbor = location.move_(dir);
            if maze.grid.get(neighbor) != Some(&Cell::Empty) {
                continue;
            }

            if let Some(&old_dist) = dist_from_start.get(&(neighbor, dir)) {
                if old_dist > new_dist {
                    dist_from_start.insert((neighbor, dir), new_dist);
                    heap.push(HeapEntry {
                        location: neighbor,
                        dist: new_dist,
                        facing: dir,
                    })
                }
            } else {
                dist_from_start.insert((neighbor, dir), new_dist);
                heap.push(HeapEntry {
                    location: neighbor,
                    dist: new_dist,
                    facing: dir,
                })
            };
        }
    }

    let shortest = *Direction::all_directions()
        .iter()
        .flat_map(|d| dist_from_start.get(&(maze.end, *d)))
        .min()
        .expect(&format!(
            "did not find a path from {:?} to {:?}",
            maze.start, maze.end
        ));

    let points = count_points_on_shortest_path(maze, &dist_from_start, shortest);

    (shortest, points)
}

fn count_points_on_shortest_path(
    maze: &Maze,
    dists: &HashMap<(Coord, Direction), usize>,
    cost: usize,
) -> usize {
    let mut path_points = HashSet::from([maze.end]);
    let mut stack = vec![
        (maze.end, Direction::Right, cost),
        (maze.end, Direction::Up, cost),
    ];

    while let Some((loc, facing, cost)) = stack.pop() {
        for dir in Direction::all_directions() {
            if dir == facing.opposite() {
                continue;
            }

            let neighbor = loc.move_(facing.opposite());
            let Some(path_cost) = dists.get(&(neighbor, dir)) else {
                continue;
            };

            let step_cost = if dir == facing { 1 } else { 1001 };

            if path_cost + step_cost == cost {
                path_points.insert(neighbor);
                stack.push((neighbor, dir, *path_cost));
            }
        }
    }

    path_points.len()
}
