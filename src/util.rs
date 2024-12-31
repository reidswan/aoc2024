#![allow(dead_code)]
use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

pub struct Counter<T: Hash + Eq> {
    counts: HashMap<T, usize>,
}

impl<T: Hash + Eq> Counter<T> {
    pub fn new() -> Self {
        Counter {
            counts: HashMap::default(),
        }
    }

    pub fn from_iter<U: Iterator<Item = T>>(items: U) -> Self {
        let mut counter = Self::new();

        counter.count_all(items);

        counter
    }

    pub fn count(&mut self, item: T) {
        self.counts.entry(item).or_default().add_assign(1);
    }

    pub fn count_n(&mut self, item: T, n: usize) {
        self.counts.entry(item).or_default().add_assign(n);
    }

    pub fn count_all<U: Iterator<Item = T>>(&mut self, items: U) {
        items.for_each(|it| self.count(it));
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, usize)> {
        self.counts.iter().map(|(t, u)| (t, *u))
    }

    #[allow(dead_code)]
    pub fn get(&self, item: &T) -> usize {
        self.counts.get(item).map(|c| *c).unwrap_or(0)
    }

    pub fn total(&self) -> usize {
        self.counts.values().sum()
    }
}

impl<T: Hash + Eq> FromIterator<T> for Counter<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Counter::from_iter(iter.into_iter())
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T>(pub Vec<Vec<T>>);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Coord(pub isize, pub isize);

impl Coord {
    pub fn move_(self, direction: Direction) -> Self {
        let Coord(x, y) = self;

        match direction {
            Direction::Up => Coord(x, y - 1),
            Direction::Down => Coord(x, y + 1),
            Direction::Left => Coord(x - 1, y),
            Direction::Right => Coord(x + 1, y),
        }
    }

    pub fn adjacent(self) -> [Coord; 4] {
        [
            self.move_(Direction::Right),
            self.move_(Direction::Left),
            self.move_(Direction::Up),
            self.move_(Direction::Down),
        ]
    }

    pub fn move_while<F: Fn(Coord) -> bool>(self, direction: Direction, predicate: F) -> Coord {
        let mut curr = self;
        while predicate(curr.move_(direction)) {
            curr = curr.move_(direction);
        }

        curr
    }

    pub fn manhattan_distance(self, other: Self) -> usize {
        (self.0 - other.0).abs() as usize + (self.1 - other.1).abs() as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all_directions() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn perpendicular_directions(self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }

    pub fn is_vertical(self) -> bool {
        self == Direction::Up || self == Direction::Down
    }

    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => return None,
        })
    }
}

pub struct CoordIter {
    x_curr: isize,
    y_curr: isize,
    width: isize,
    height: isize,
}

pub struct GridItemIter<'a, T> {
    grid: &'a Grid<T>,
    coord_iter: CoordIter,
}

impl<T> Grid<T> {
    pub fn get(&self, Coord(x, y): Coord) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;
        if y >= self.0.len() || x >= self.0[y].len() {
            return None;
        }

        return Some(&self.0[y][x]);
    }

    pub fn iter_coords(&self) -> CoordIter {
        CoordIter {
            x_curr: -1,
            y_curr: 0,
            height: self.0.len() as isize,
            width: if self.0.len() > 0 {
                self.0[0].len() as isize
            } else {
                0
            },
        }
    }

    pub fn set(&mut self, Coord(x, y): Coord, val: T) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        let x = x as usize;
        let y = y as usize;
        if y >= self.0.len() || x >= self.0[y].len() {
            return false;
        }

        self.0[y][x] = val;

        true
    }

    pub fn iter<'a>(&'a self) -> GridItemIter<'a, T> {
        GridItemIter {
            grid: &self,
            coord_iter: self.iter_coords(),
        }
    }
}

impl Iterator for CoordIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        self.x_curr += 1;
        if self.x_curr >= self.width {
            self.x_curr = 0;
            self.y_curr += 1;
        }

        if self.y_curr >= self.height {
            None
        } else {
            Some(Coord(self.x_curr, self.y_curr))
        }
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn with_dimensions(width: usize, height: usize) -> Self {
        Grid(
            (0..height)
                .map(|_| (0..width).map(|_| T::default()).collect())
                .collect(),
        )
    }
}

impl<'a, T> Iterator for GridItemIter<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let coord = self.coord_iter.next()?;

        self.grid.get(coord).map(|it| (coord, it))
    }
}

pub fn parse_char_grid(input: &str) -> Grid<char> {
    Grid(input.lines().map(|line| line.chars().collect()).collect())
}

pub fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ratio {
    pub numerator: isize,
    pub denominator: isize,
}

impl Ratio {
    pub fn new(numerator: isize, denominator: isize) -> Self {
        Ratio {
            numerator,
            denominator,
        }
    }

    fn reduce(self) -> Self {
        if self.denominator == 0 {
            self
        } else if self.numerator == 0 {
            Ratio {
                numerator: 0,
                denominator: 1,
            }
        } else {
            let div = gcd(self.numerator, self.denominator);

            Ratio {
                numerator: self.numerator / div,
                denominator: self.denominator / div,
            }
        }
    }

    pub fn is_negative(self) -> bool {
        (self.numerator < 0) != (self.denominator < 0)
    }

    pub fn is_multiple_of(self, other: Ratio) -> bool {
        let Ratio {
            numerator: a,
            denominator: b,
        } = self;
        let Ratio {
            numerator: c,
            denominator: d,
        } = other;

        if a % c != 0 || b % d != 0 {
            false
        } else {
            let k1 = a / c;
            let k2 = b / d;

            k1 % k2 == 0
        }
    }

    pub fn is_integer(self) -> bool {
        self.numerator % self.denominator == 0
    }

    pub fn as_integer(self) -> isize {
        self.numerator / self.denominator
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.reduce();
        write!(f, "{}/{}", r.numerator, r.denominator)
    }
}

impl Into<isize> for Ratio {
    fn into(self) -> isize {
        self.numerator / self.denominator
    }
}

impl Add<Ratio> for Ratio {
    type Output = Ratio;

    fn add(self, rhs: Ratio) -> Self::Output {
        let Ratio {
            numerator: a,
            denominator: b,
        } = self;
        let Ratio {
            numerator: c,
            denominator: d,
        } = rhs;

        Ratio {
            numerator: a * d + b * c,
            denominator: b * d,
        }
        .reduce()
    }
}

impl Neg for Ratio {
    type Output = Ratio;

    fn neg(self) -> Self::Output {
        Ratio {
            numerator: -1 * self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Sub<Ratio> for Ratio {
    type Output = Ratio;

    fn sub(self, rhs: Ratio) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl Mul<Ratio> for Ratio {
    type Output = Ratio;

    fn mul(self, rhs: Ratio) -> Self::Output {
        Ratio {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl From<isize> for Ratio {
    fn from(value: isize) -> Self {
        Ratio::new(value, 1)
    }
}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        let lhs = &self.reduce();
        let rhs = other.reduce();

        lhs.numerator == rhs.numerator && lhs.denominator == rhs.denominator
    }
}

impl Div<Ratio> for Ratio {
    type Output = Ratio;

    fn div(self, rhs: Ratio) -> Self::Output {
        let Ratio {
            numerator: a,
            denominator: b,
        } = self;
        let Ratio {
            numerator: c,
            denominator: d,
        } = rhs;

        Ratio {
            numerator: a * d,
            denominator: b * c,
        }
    }
}

pub struct ComboIter<'a, T> {
    options: &'a [T],
    len: usize,
    curr: usize,
}

pub fn all_combos_with_length<T: Copy + Default>(options: &[T], len: usize) -> ComboIter<T> {
    ComboIter {
        options,
        len,
        curr: 0,
    }
}

impl<'a, T> Iterator for ComboIter<'a, T>
where
    T: Copy + Default,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.options.len().pow(self.len as u32) {
            return None;
        }

        let mut curr = self.curr;
        self.curr += 1;

        let mut combo = vec![T::default(); self.len];
        for ix in 0..self.len {
            combo[ix] = self.options[curr % self.options.len()];
            curr = curr / self.options.len();
        }

        Some(combo)
    }
}

pub fn permutations<T: Copy>(src: &mut [T], n: usize) -> Vec<Vec<T>> {
    if n <= 1 {
        return vec![src.iter().map(|i| *i).collect()];
    }

    let mut perms = vec![];

    perms.extend(permutations(src, n - 1));

    for i in 0..n - 1 {
        if n % 2 == 0 {
            src.swap(i, n - 1)
        } else {
            src.swap(0, n - 1)
        }
        perms.extend(permutations(src, n - 1));
    }

    perms
}
