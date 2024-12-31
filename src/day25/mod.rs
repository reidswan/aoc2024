const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let input = parse_input(INPUT);
    println!("Part 1: {}", count_potential_matches(&input));
}

fn count_potential_matches(input: &Input) -> usize {
    let mut count = 0;
    for key in &input.keys {
        for lock in &input.locks {
            if key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 6) {
                count += 1
            }
        }
    }

    count
}

#[derive(Eq, PartialEq)]
enum SchematicType {
    Key,
    Lock,
}

#[derive(Debug)]
struct Input {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}

fn parse_input(input: &str) -> Input {
    let mut keys = vec![];
    let mut locks = vec![];
    for schematic_raw in input.split("\n\n") {
        let first_ch = schematic_raw.chars().next().unwrap();
        let st = if first_ch == '.' {
            SchematicType::Key
        } else {
            SchematicType::Lock
        };

        let mut heights = [0u8; 5];
        for (i, line) in schematic_raw.trim().lines().enumerate() {
            for (j, c) in line.trim().chars().enumerate() {
                if heights[j] == 0 && c != first_ch {
                    heights[j] = if st == SchematicType::Key {
                        6 - i as u8
                    } else {
                        i as u8
                    };
                }
            }
        }

        if st == SchematicType::Key {
            keys.push(heights)
        } else {
            locks.push(heights)
        }
    }

    Input { keys, locks }
}
