const INPUT: &str = include_str!("./input.in");

pub fn run() {
    part_one();
    part_two();
}

type Warehouse = Vec<Vec<Cell>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Empty,
    Crate,
    BigCrateLeft,
    BigCrateRight,
}

use crate::util::Direction::{self, *};

fn parse_input(input: &str) -> (Warehouse, (usize, usize), Vec<Direction>) {
    let mut robot_pos = (0, 0);

    let (warehouse_raw, directions_raw) = input.trim().split_once("\n\n").unwrap();

    let warehouse = warehouse_raw
        .trim()
        .lines()
        .enumerate()
        .map(|(line_no, line)| {
            line.chars()
                .enumerate()
                .map(|(col_no, c)| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'O' => Cell::Crate,
                    '@' => {
                        robot_pos = (line_no, col_no);
                        Cell::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let directions = directions_raw
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|e| match e {
                    '<' => Left,
                    '^' => Up,
                    'v' => Down,
                    '>' => Right,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (warehouse, robot_pos, directions)
}

fn try_move(dir: Direction, position: (usize, usize), warehouse: &mut Warehouse) -> bool {
    let neighbor_pos = move_(position, dir);
    match warehouse[neighbor_pos.0][neighbor_pos.1] {
        Cell::Empty => {
            warehouse[neighbor_pos.0][neighbor_pos.1] = warehouse[position.0][position.1];
            warehouse[position.0][position.1] = Cell::Empty;
            true
        }
        Cell::Crate => {
            if try_move(dir, neighbor_pos, warehouse) {
                warehouse[neighbor_pos.0][neighbor_pos.1] = warehouse[position.0][position.1];
                warehouse[position.0][position.1] = Cell::Empty;
                true
            } else {
                false
            }
        }
        Cell::Wall => false,
        _ => unreachable!(),
    }
}

fn part_one() {
    let (mut warehouse, mut position, directions) = parse_input(INPUT);

    for dir in directions {
        if try_move(dir, position, &mut warehouse) {
            position = match dir {
                Up => (position.0 - 1, position.1),
                Down => (position.0 + 1, position.1),
                Left => (position.0, position.1 - 1),
                Right => (position.0, position.1 + 1),
            };
        }
    }

    println!("Part 1: {}", calc_gps(&warehouse, Cell::Crate));
}

fn enlarge_warehouse(small_warehouse: Warehouse) -> Warehouse {
    let mut larger_warehouse = vec![];
    for line in small_warehouse {
        let mut larger_line = vec![];
        for tile in line {
            if let Cell::Crate = tile {
                larger_line.push(Cell::BigCrateLeft);
                larger_line.push(Cell::BigCrateRight);
            } else {
                larger_line.push(tile);
                larger_line.push(tile);
            }
        }
        larger_warehouse.push(larger_line);
    }
    larger_warehouse
}

fn move_((line, col): (usize, usize), dir: Direction) -> (usize, usize) {
    match dir {
        Up => (line - 1, col),
        Down => (line + 1, col),
        Left => (line, col - 1),
        Right => (line, col + 1),
    }
}

fn try_move_larger_crates(
    dir: Direction,
    position: (usize, usize),
    warehouse: &mut Warehouse,
    apply: bool,
) -> bool {
    let neighbor_pos = move_(position, dir);

    match warehouse[neighbor_pos.0][neighbor_pos.1] {
        Cell::Empty => {
            if apply {
                warehouse[neighbor_pos.0][neighbor_pos.1] = warehouse[position.0][position.1];
                warehouse[position.0][position.1] = Cell::Empty;
            }
            true
        }
        Cell::Wall => false,
        Cell::BigCrateLeft => match dir {
            Up | Down => {
                if try_move_larger_crates(dir, neighbor_pos, warehouse, apply)
                    & try_move_larger_crates(
                        dir,
                        (neighbor_pos.0, neighbor_pos.1 + 1),
                        warehouse,
                        apply,
                    )
                {
                    if apply {
                        warehouse[neighbor_pos.0][neighbor_pos.1] =
                            warehouse[position.0][position.1];
                        warehouse[position.0][position.1] = Cell::Empty;
                    }
                    true
                } else {
                    false
                }
            }
            _ => {
                if try_move_larger_crates(dir, neighbor_pos, warehouse, apply) {
                    if apply {
                        warehouse[neighbor_pos.0][neighbor_pos.1] =
                            warehouse[position.0][position.1];
                        warehouse[position.0][position.1] = Cell::Empty;
                    }
                    true
                } else {
                    false
                }
            }
        },
        Cell::BigCrateRight => match dir {
            Up | Down => {
                if try_move_larger_crates(dir, neighbor_pos, warehouse, apply)
                    & try_move_larger_crates(
                        dir,
                        (neighbor_pos.0, neighbor_pos.1 - 1),
                        warehouse,
                        apply,
                    )
                {
                    if apply {
                        warehouse[neighbor_pos.0][neighbor_pos.1] =
                            warehouse[position.0][position.1];
                        warehouse[position.0][position.1] = Cell::Empty;
                    }
                    true
                } else {
                    false
                }
            }
            _ => {
                if try_move_larger_crates(dir, neighbor_pos, warehouse, apply) {
                    if apply {
                        warehouse[neighbor_pos.0][neighbor_pos.1] =
                            warehouse[position.0][position.1];
                        warehouse[position.0][position.1] = Cell::Empty;
                    }
                    true
                } else {
                    false
                }
            }
        },
        _ => unreachable!(),
    }
}

fn part_two() {
    let (warehouse, mut position, directions) = parse_input(INPUT);
    let mut larger_warehouse = enlarge_warehouse(warehouse);
    position.1 *= 2;

    for dir in directions {
        if try_move_larger_crates(dir, position, &mut larger_warehouse, false) {
            try_move_larger_crates(dir, position, &mut larger_warehouse, true);
            position = match dir {
                Up => (position.0 - 1, position.1),
                Down => (position.0 + 1, position.1),
                Left => (position.0, position.1 - 1),
                Right => (position.0, position.1 + 1),
            };
        }
    }

    println!(
        "Part 2: {}",
        calc_gps(&larger_warehouse, Cell::BigCrateLeft)
    );
}

fn calc_gps(wh: &Warehouse, target_tile: Cell) -> usize {
    wh.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(
                    |(x, tile)| {
                        if *tile == target_tile {
                            100 * y + x
                        } else {
                            0
                        }
                    },
                )
                .sum::<usize>()
        })
        .sum()
}
