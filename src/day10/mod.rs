use std::collections::HashSet;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let map = parse_input(INPUT);
    let trail_heads = find_trail_heads(&map);
    let total_score = trail_heads
        .iter()
        .map(|th| trail_head_score(&map, *th))
        .sum::<usize>();

    println!("Part 1: {}", total_score);

    let total_rating = trail_heads
        .iter()
        .map(|th| trail_head_rating(&map, *th))
        .sum::<usize>();

    println!("Part 1: {}", total_rating);
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

type Coord = (usize, usize);

fn find_trail_heads(map: &Vec<Vec<usize>>) -> Vec<Coord> {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, &height)| {
                    if height == 0 {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .flatten()
        .collect()
}

fn trail_head_score(map: &Vec<Vec<usize>>, start: Coord) -> usize {
    let mut score = 0;
    let mut stack = vec![start];
    let mut visited = HashSet::new();

    while let Some((x, y)) = stack.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        let curr = map[y][x];
        if curr == 9 {
            score += 1;
            continue;
        }

        let nexts = [
            (x as isize, y as isize - 1),
            (x as isize - 1, y as isize),
            (x as isize, y as isize + 1),
            (x as isize + 1, y as isize),
        ];

        for next in nexts {
            if let Some(v) = get(map, next) {
                if v == curr + 1 {
                    stack.push((next.0 as usize, next.1 as usize))
                }
            }
        }
    }
    score
}

fn get<T: Copy>(src: &Vec<Vec<T>>, (x, y): (isize, isize)) -> Option<T> {
    if x < 0 || y < 0 {
        return None;
    }

    let (x, y) = (x as usize, y as usize);

    if y < src.len() && x < src[y].len() {
        Some(src[y][x])
    } else {
        None
    }
}

fn trail_head_rating(map: &Vec<Vec<usize>>, start: Coord) -> usize {
    let mut score = 0;
    let mut stack = vec![start];

    while let Some((x, y)) = stack.pop() {
        let curr = map[y][x];
        if curr == 9 {
            score += 1;
            continue;
        }

        if y > 0 && map[y - 1][x] == curr + 1 {
            stack.push((x, y - 1))
        }
        if x > 0 && map[y][x - 1] == curr + 1 {
            stack.push((x - 1, y))
        }
        if y + 1 < map.len() && map[y + 1][x] == curr + 1 {
            stack.push((x, y + 1));
        }
        if x + 1 < map[y].len() && map[y][x + 1] == curr + 1 {
            stack.push((x + 1, y))
        }
    }
    score
}
