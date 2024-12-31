use std::collections::HashSet;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let input = INPUT
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    println!("Part 1: {}", count_xmas(&input));
    println!("Part 2: {}", count_x_mas(&input));
}

type Coord = (isize, isize);

const fn dir_add((l0, l1): Coord, (r0, r1): Coord) -> Coord {
    (l0 + r0, l1 + r1)
}

const LEFT: Coord = (-1, 0);
const RIGHT: Coord = (1, 0);
const UP: Coord = (0, -1);
const DOWN: Coord = (0, 1);
const UP_LEFT: Coord = dir_add(UP, LEFT);
const UP_RIGHT: Coord = dir_add(UP, RIGHT);
const DOWN_LEFT: Coord = dir_add(DOWN, LEFT);
const DOWN_RIGHT: Coord = dir_add(DOWN, RIGHT);

const ALL_DIRECTIONS: [Coord; 8] = [
    LEFT, RIGHT, UP, DOWN, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
];

const DIAGONAL_DIRECTIONS: [Coord; 4] = [UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT];

pub fn count_xmas(input: &Vec<Vec<char>>) -> usize {
    GridIter::from(input)
        .filter(|coord| {
            get_coord(input, *coord)
                .map(|it| it == 'X')
                .unwrap_or(false)
        })
        .map(|coord| {
            ALL_DIRECTIONS
                .into_iter()
                .filter(|dir| match_direction(input, "XMAS", *dir, coord))
                .count()
        })
        .sum()
}

struct GridIter {
    width: usize,
    height: usize,
    x_curr: usize,
    y_curr: usize,
}

impl GridIter {
    fn from<T>(src: &Vec<Vec<T>>) -> Self {
        GridIter {
            width: src[0].len(),
            height: src.len(),
            x_curr: 0,
            y_curr: 0,
        }
    }

    fn incr(&mut self) {
        self.x_curr += 1;
        if self.x_curr >= self.width {
            self.x_curr = 0;
            self.y_curr += 1;
        }
    }
}

impl Iterator for GridIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y_curr >= self.height {
            return None;
        }

        let res = (self.x_curr as isize, self.y_curr as isize);

        self.incr();

        Some(res)
    }
}

pub fn count_x_mas(input: &Vec<Vec<char>>) -> usize {
    let mut centres_of_mas = HashSet::<Coord>::new();

    GridIter::from(input)
        .filter(|coord| {
            get_coord(input, *coord)
                .map(|it| it == 'M')
                .unwrap_or(false)
        })
        .map(|(x, y)| {
            let start = (x as isize, y as isize);
            let new_centres = DIAGONAL_DIRECTIONS
                .into_iter()
                .filter(|&dir| match_direction(input, "MAS", dir, start))
                .map(|coord| dir_add(coord, start))
                .collect::<Vec<_>>();

            let cross_count = new_centres
                .iter()
                .filter(|&it| centres_of_mas.contains(it))
                .count();

            for c in new_centres {
                centres_of_mas.insert(c);
            }

            cross_count
        })
        .sum()
}

pub fn get_coord(src: &Vec<Vec<char>>, (x, y): Coord) -> Option<char> {
    src.get(y as usize)
        .and_then(|line| line.get(x as usize))
        .map(|c| *c)
}

pub fn match_direction(src: &Vec<Vec<char>>, target: &str, direction: Coord, start: Coord) -> bool {
    let (x, y) = start;
    let (dx, dy) = direction;

    for (i, target_c) in target.chars().enumerate() {
        let x_c = x + i as isize * dx;
        let y_c = y + i as isize * dy;

        match get_coord(src, (x_c, y_c)) {
            Some(ch) if ch == target_c => {}
            _ => return false,
        }
    }

    true
}
