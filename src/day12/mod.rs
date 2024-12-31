use std::collections::HashSet;

use crate::util::{parse_char_grid, Coord, Direction, Grid};

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let grid = parse_char_grid(INPUT);
    let regions = find_contiguous_regions(&grid);
    let cost_p1 = regions
        .iter()
        .map(|r| r.area.len() * count_fence_pieces(r))
        .sum::<usize>();

    println!("Part 1: {}", cost_p1);

    let cost_p2 = regions
        .iter()
        .map(|r| r.area.len() * count_fence_lines(r))
        .sum::<usize>();

    println!("Part 2: {}", cost_p2);
}

struct Region {
    area: HashSet<Coord>,
    edges: HashSet<Coord>,
}

fn find_contiguous_regions(grid: &Grid<char>) -> Vec<Region> {
    let mut visited = HashSet::new();
    let mut regions = vec![];
    for coord in grid.iter_coords() {
        if visited.contains(&coord) {
            continue;
        }

        visited.insert(coord);
        let Some(region) = find_contiguous_region(grid, coord) else {
            continue;
        };

        visited.extend(&region.area);
        regions.push(region)
    }

    regions
}

fn find_contiguous_region(grid: &Grid<char>, start: Coord) -> Option<Region> {
    let Some(&region_type) = grid.get(start) else {
        return None;
    };

    let mut area = HashSet::new();
    let mut stack = vec![start];
    while let Some(coord) = stack.pop() {
        area.insert(coord);

        for dir in Direction::all_directions() {
            let next = coord.move_(dir);
            if area.contains(&next) {
                continue;
            }

            match grid.get(next) {
                Some(&c) if c == region_type => stack.push(next),
                _ => {}
            }
        }
    }

    let edges = find_edges(&area);

    Some(Region { area, edges })
}

fn find_edges(region_area: &HashSet<Coord>) -> HashSet<Coord> {
    region_area
        .iter()
        .filter(|square| {
            Direction::all_directions()
                .into_iter()
                .any(|dir| !region_area.contains(&square.move_(dir)))
        })
        .map(|c| *c)
        .collect()
}

fn count_fence_pieces(region: &Region) -> usize {
    region
        .edges
        .iter()
        .map(|edge| {
            Direction::all_directions()
                .iter()
                .filter(|&&direction| !region.area.contains(&edge.move_(direction)))
                .count()
        })
        .sum()
}

fn count_fence_lines(region: &Region) -> usize {
    region
        .edges
        .iter()
        .flat_map(|edge| get_lines_including(*edge, &region))
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Line {
    direction: Direction,
    start: Coord,
    end: Coord,
}

fn get_lines_including(edge: Coord, region: &Region) -> Vec<Line> {
    Direction::all_directions()
        .iter()
        .filter_map(|&direction| {
            // Is this an edge for the given direction?
            // i.e. if we move in the given direction, do we exit the region?
            if region.area.contains(&edge.move_(direction)) {
                return None;
            }

            let [to_start, to_end] = direction.perpendicular_directions();

            let start = edge.move_while(to_start, |coord| {
                // If we move `to_start`, are we still on an edge for `direction`?
                region.edges.contains(&coord) && !region.area.contains(&coord.move_(direction))
            });

            let end = edge.move_while(to_end, |coord| {
                // If we move `to_end`, are we still on an edge for `direction`?
                region.edges.contains(&coord) && !region.area.contains(&coord.move_(direction))
            });

            Some(Line {
                direction,
                start,
                end,
            })
        })
        .collect()
}
