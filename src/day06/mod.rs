use std::collections::HashSet;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let (mut map, guard_loc) = parse_input(INPUT);
    let init_direction = (0, -1);
    let visited = trace_path(&map, guard_loc, init_direction);

    println!("Part 1: {}", visited.len());

    println!(
        "Part 2: {}",
        make_loops(&mut map, guard_loc, init_direction, &visited)
    )
}

#[derive(PartialEq, Eq)]
enum Slot {
    Empty,
    Obstacle,
}

type Coord = (usize, usize);
type Direction = (isize, isize);

fn parse_input(input: &str) -> (Vec<Vec<Slot>>, Coord) {
    let mut grid = Vec::with_capacity(input.lines().count());
    let mut guard_location = (0, 0);

    for line in input.lines() {
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            let slot = match c {
                '.' => Slot::Empty,
                '#' => Slot::Obstacle,
                '^' => {
                    guard_location = (row.len(), grid.len());
                    Slot::Empty
                }
                _ => panic!("unrecognized grid char: {}", c),
            };

            row.push(slot);
        }

        grid.push(row);
    }

    (grid, guard_location)
}

fn rotate_direction((a, b): Direction) -> Direction {
    (-1 * b, a)
}

fn trace_path(
    map: &Vec<Vec<Slot>>,
    mut guard_loc: Coord,
    mut direction: Direction,
) -> HashSet<Coord> {
    let mut visited = HashSet::new();

    loop {
        visited.insert(guard_loc);

        let (guard_x, guard_y) = guard_loc;
        let (dir_x, dir_y) = direction;

        let y_next = guard_y as isize + dir_y;
        if y_next < 0 || y_next as usize >= map.len() {
            break;
        }
        let y_next = y_next as usize;

        let x_next = guard_x as isize + dir_x;
        if x_next < 0 || x_next as usize >= map[y_next as usize].len() {
            break;
        }
        let x_next = x_next as usize;

        match map[y_next][x_next] {
            Slot::Obstacle => {
                direction = rotate_direction(direction);
            }
            Slot::Empty => {
                guard_loc = (x_next, y_next);
            }
        };
    }

    visited
}

fn make_loops(
    map: &mut Vec<Vec<Slot>>,
    guard_loc: Coord,
    direction: Direction,
    visited_locations: &HashSet<Coord>,
) -> usize {
    let mut count = 0;

    for &(x, y) in visited_locations {
        if (x, y) == guard_loc {
            continue;
        }
        map[y][x] = Slot::Obstacle;

        if check_loop(&map, guard_loc, direction) {
            count += 1;
        }

        map[y][x] = Slot::Empty;
    }

    count
}

fn check_loop(map: &Vec<Vec<Slot>>, mut guard_loc: Coord, mut direction: Direction) -> bool {
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&(guard_loc, direction)) {
            return true;
        } else {
            visited.insert((guard_loc, direction));
        }

        let (guard_x, guard_y) = guard_loc;
        let (dir_x, dir_y) = direction;

        let y_next = guard_y as isize + dir_y;
        if y_next < 0 || y_next as usize >= map.len() {
            return false;
        }
        let y_next = y_next as usize;

        let x_next = guard_x as isize + dir_x;
        if x_next < 0 || x_next as usize >= map[y_next as usize].len() {
            return false;
        }
        let x_next = x_next as usize;

        match map[y_next][x_next] {
            Slot::Obstacle => {
                direction = rotate_direction(direction);
            }
            Slot::Empty => {
                guard_loc = (x_next, y_next);
            }
        };
    }
}
