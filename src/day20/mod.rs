use std::collections::{HashMap, HashSet};

use crate::util::{Coord, Direction, Grid};

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let track = parse_input(INPUT);
    println!("Part 1: {}", find_cheats(&track).len());
    println!("Part 2: {}", find_cheats_p2(&track, 100).len());
}

#[derive(Debug)]
enum Cell {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Track {
    path: Vec<Coord>,
}

fn parse_input(input: &str) -> Track {
    let mut start = None;
    let mut end = None;
    let map = Grid(
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Some(Coord(x as isize, y as isize));
                            Cell::Empty
                        }
                        'E' => {
                            end = Some(Coord(x as isize, y as isize));
                            Cell::Empty
                        }
                        '#' => Cell::Wall,
                        '.' => Cell::Empty,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    );

    let start = start.expect("did not find start");
    let end = end.expect("did not find end");
    let path = get_path(&map, start, end);

    Track { path }
}

fn get_path(map: &Grid<Cell>, start: Coord, end: Coord) -> Vec<Coord> {
    let mut curr = start;
    let mut prev_dir = None;
    let mut path = vec![start];

    'pathloop: while curr != end {
        for direction in Direction::all_directions() {
            if Some(direction.opposite()) == prev_dir {
                continue;
            }

            let next = curr.move_(direction);
            if path.contains(&next) {
                continue;
            }

            if let Some(Cell::Empty) = map.get(next) {
                curr = next;
                prev_dir = Some(direction);
                path.push(curr);
                continue 'pathloop;
            }
        }

        for direction in Direction::all_directions() {
            let next = curr.move_(direction);
            if path.contains(&next) {
                println!("{:?} is already in the path!", next);
            } else {
                println!("{:?} = {:?}", next, map.get(next));
            }
        }

        panic!("{:?}", curr);
    }

    path
}

fn reverse_index(path: &[Coord]) -> HashMap<Coord, usize> {
    path.iter().enumerate().map(|(i, c)| (*c, i)).collect()
}

fn find_cheats(track: &Track) -> HashSet<(Coord, Coord)> {
    let reverse_index = reverse_index(&track.path);
    let mut good_cheats = HashSet::new();

    for (i, loc) in track.path[..track.path.len() - 100].iter().enumerate() {
        for dir1 in Direction::all_directions() {
            for dir2 in Direction::all_directions() {
                if dir2 == dir1.opposite() {
                    continue;
                }

                let next = loc.move_(dir1).move_(dir2);
                let Some(&next_i) = reverse_index.get(&next) else {
                    continue;
                };

                if next_i > i && next_i - i > 100 {
                    good_cheats.insert((*loc, next));
                }
            }
        }
    }

    good_cheats
}

fn find_cheats_p2(track: &Track, min_save: usize) -> HashSet<(Coord, Coord)> {
    let mut good_cheats = HashSet::new();
    for (i, loc) in track.path[..track.path.len() - min_save].iter().enumerate() {
        for (j, next_loc) in track.path[i + min_save..].iter().enumerate() {
            let dist = next_loc.manhattan_distance(*loc);
            if next_loc.manhattan_distance(*loc) <= 20 && j >= dist {
                good_cheats.insert((*loc, *next_loc));
            }
        }
    }

    good_cheats
}
