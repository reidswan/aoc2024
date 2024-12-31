use std::{collections::HashMap, str::FromStr};

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let input = parse_input(INPUT);
    println!("Part 1: {}", simulate(&input));
}

#[derive(Debug, Clone, Copy)]
enum GateType {
    AND,
    OR,
    XOR,
}

impl FromStr for GateType {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "AND" => GateType::AND,
            "OR" => GateType::OR,
            "XOR" => GateType::XOR,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    left: &'a str,
    right: &'a str,
    output: &'a str,
    gate: GateType,
}

#[derive(Debug, Clone)]
struct Input<'a> {
    initial_values: HashMap<&'a str, bool>,
    gates: Vec<Gate<'a>>,
    output_gate_map: HashMap<&'a str, Gate<'a>>,
}

fn parse_input<'a>(input: &'a str) -> Input<'a> {
    let (initial_values_raw, gates_raw) = input.split_once("\n\n").unwrap();

    let initial_values = initial_values_raw
        .trim()
        .lines()
        .map(|line| {
            let (wire, val) = line.split_once(": ").unwrap();

            (wire, val.trim() == "1")
        })
        .collect();

    let gates: Vec<Gate<'_>> = gates_raw
        .trim()
        .lines()
        .map(|line| {
            let (inputs, output) = line.split_once("->").unwrap();
            let mut input_parts = inputs.trim().split(" ");
            let left = input_parts.next().unwrap();
            let gate = input_parts.next().unwrap().parse::<GateType>().unwrap();
            let right = input_parts.next().unwrap();

            Gate {
                left,
                right,
                output: output.trim(),
                gate,
            }
        })
        .collect();

    let mut output_gate_map = HashMap::new();
    for gate in &gates {
        output_gate_map.insert(gate.output, *gate);
    }

    Input {
        initial_values,
        gates,
        output_gate_map,
    }
}

fn simulate(input: &Input) -> usize {
    let mut values = input.initial_values.clone();

    let mut needed_values: Vec<_> = input
        .gates
        .iter()
        .filter(|gate| gate.output.starts_with('z'))
        .map(|gate| {
            (
                gate.output,
                compute_value(gate.output, &input.output_gate_map, &mut values),
            )
        })
        .collect();

    needed_values.sort_by_key(|(a, _)| -1 * a.replace('z', "").parse::<isize>().unwrap());

    let mut res = 0;
    for (_, v) in needed_values {
        res = res * 2 + if v { 1 } else { 0 };
    }

    res
}

fn compute_value<'a>(
    wire: &'a str,
    output_gates: &HashMap<&'a str, Gate<'a>>,
    values: &mut HashMap<&'a str, bool>,
) -> bool {
    if values.contains_key(wire) {
        return *values.get(wire).unwrap();
    }

    let gate = output_gates.get(wire).unwrap();

    let lval = compute_value(gate.left, output_gates, values);
    let rval = compute_value(gate.right, output_gates, values);
    let output = match gate.gate {
        GateType::AND => lval && rval,
        GateType::OR => lval || rval,
        GateType::XOR => lval ^ rval,
    };

    values.insert(wire, output);

    output
}

#[allow(dead_code)]
impl<'a> Input<'a> {
    fn set_x(&mut self, mut value: usize) {
        for i in 0..45 {
            *self
                .initial_values
                .get_mut(format!("x{:02}", i).as_str())
                .unwrap() = (value % 2) == 1;
            value /= 2;
        }
    }

    fn set_y(&mut self, mut value: usize) {
        for i in 0..45 {
            *self
                .initial_values
                .get_mut(format!("y{:02}", i).as_str())
                .unwrap() = (value % 2) == 1;
            value /= 2;
        }
    }

    fn make_dot(&self) -> String {
        let mut dotfile = String::new();
        dotfile.push_str("digraph G {\n");

        for &Gate {
            left,
            right,
            output,
            gate,
        } in self.gates.iter()
        {
            let (op, color) = match gate {
                GateType::AND => ("&", "red"),
                GateType::OR => ("|", "blue"),
                GateType::XOR => ("^", "green"),
            };
            let gate_node = format!("\"{left} {op} {right}\"");
            dotfile.push_str(format!("{gate_node} [color={color} fontcolor={color}]\n").as_str());
            dotfile.push_str(format!("{left} -> {gate_node}\n").as_str());
            dotfile.push_str(format!("{right} -> {gate_node}\n").as_str());
            dotfile.push_str(format!("{gate_node} -> {output}\n").as_str());
            if output.starts_with("z") {
                dotfile.push_str(format!("{output} [rank=10000]\n").as_str());
            }
        }

        dotfile.push_str("\n}");
        dotfile
    }

    fn find_wrong_bits(&mut self) -> Vec<i32> {
        let mut wrong: Vec<(usize, usize, usize)> = vec![];
        for x in 0..45usize {
            self.set_x(1 << x);
            for y in 0..45usize {
                self.set_y(1 << y);

                let z = simulate(&self);
                if z != (1 << x) + (1 << y) {
                    wrong.push((1 << x, 1 << y, z));
                }
            }
        }

        let mut wrong = wrong.iter().fold(0, |acc, (x, y, z)| acc | ((x + y) ^ z));
        let mut wrong_bits = vec![];
        let mut i = 0;
        while wrong != 0 {
            if wrong % 2 == 1 {
                wrong_bits.push(i)
            }
            wrong = wrong / 2;
            i += 1
        }

        wrong_bits
    }
}
