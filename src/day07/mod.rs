const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let equations = parse_input(INPUT);

    let (succeeded, failed) = equations.iter().partition::<Vec<_>, _>(|eq| can_equate(eq));

    let succeeded_tot = succeeded.iter().map(|it| it.target).sum::<usize>();
    println!("Part 1: {}", succeeded_tot);
    println!(
        "Part 2: {}",
        succeeded_tot
            + failed
                .iter()
                .filter(|eq| can_equate_p2(eq))
                .map(|it| it.target)
                .sum::<usize>()
    );
}

struct Input {
    target: usize,
    operands: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let Some((target, rest)) = line.split_once(": ") else {
                panic!("Bad input: {}", line);
            };
            let target = target.parse::<usize>().unwrap();

            let operands = rest
                .split(" ")
                .map(|it| it.parse::<usize>().unwrap())
                .collect();

            Input {
                target: target,
                operands: operands,
            }
        })
        .collect()
}

fn can_equate(input: &Input) -> bool {
    let combos = 2usize.pow(input.operands.len() as u32 - 1);

    'outer: for i in 0..combos {
        let mut pat = i;
        let mut remaining = input.target;

        for &op in input.operands.iter().rev() {
            if pat % 2 == 0 && op <= remaining {
                remaining -= op;
            } else if pat % 2 == 1 && remaining % op == 0 {
                remaining = remaining / op;
            } else {
                continue 'outer;
            }

            pat = pat / 2;
        }
        if remaining == 0 {
            return true;
        }
    }

    false
}

fn can_equate_p2(input: &Input) -> bool {
    let combos = 3usize.pow(input.operands.len() as u32 - 1);

    for i in 0..combos {
        let mut pat = i;
        let mut remaining = input.target;

        'outer: for &op in input.operands.iter().rev() {
            if pat % 3 == 0 {
                let Some(res) = remove_matching_end(remaining, op) else {
                    continue 'outer;
                };
                remaining = res;
            } else if pat % 3 == 1 && remaining % op == 0 {
                remaining = remaining / op;
            } else if pat % 3 == 2 && op <= remaining {
                remaining -= op;
            } else {
                continue 'outer;
            }

            pat = pat / 3;
        }

        if remaining == 0 {
            return true;
        }
    }

    false
}

fn remove_matching_end(num: usize, end: usize) -> Option<usize> {
    if end > num {
        return None;
    } else if end == num {
        return Some(0);
    }

    let end_len = (end as f64).log10().floor() as u32;

    let fac = 10usize.pow(end_len + 1);
    let res = (num - end) / fac;

    if res * fac + end == num {
        Some(res)
    } else {
        None
    }
}
