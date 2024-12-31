const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let mut computer = parse_input(INPUT);

    computer.run();

    println!(
        "Part 1: {}",
        computer
            .output
            .iter()
            .map(|o| format!("{}", o))
            .collect::<Vec<_>>()
            .join(",")
    );

    let instructions = computer.instructions.clone();
    let mut options = vec![0];
    for i in (0..instructions.len()).rev() {
        options = computer.find_values_outputting(&instructions[i..instructions.len()], &options);
    }

    println!("Part 2: {:?}", options.iter().min().unwrap());
}

#[derive(Debug, Clone)]
struct Computer {
    instructions: Vec<u128>,
    a: u128,
    b: u128,
    c: u128,
    ip: usize,
    output: Vec<u128>,
}

impl Computer {
    fn run(&mut self) {
        while self.ip + 1 < self.instructions.len() {
            let instr = self.instructions[self.ip];
            let op = self.instructions[self.ip + 1];
            let mut jump_to = self.ip + 2;
            match instr {
                0 => self.adv(op),
                1 => self.bxl(op),
                2 => self.bst(op),
                3 => {
                    if let Some(jmp) = self.jnz(op) {
                        jump_to = jmp as usize;
                    }
                }
                4 => self.bxc(op),
                5 => self.out(op),
                6 => self.bdv(op),
                7 => self.cdv(op),
                _ => unreachable!(),
            }
            self.ip = jump_to
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.output = vec![];
    }

    fn combo_value(&self, v: u128) -> u128 {
        match v {
            0..=3 => v,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn adv(&mut self, op: u128) {
        self.a >>= self.combo_value(op);
    }

    fn bxl(&mut self, op: u128) {
        self.b ^= op;
    }

    fn bst(&mut self, op: u128) {
        self.b = self.combo_value(op) % 8;
    }

    fn jnz(&mut self, op: u128) -> Option<u128> {
        if self.a == 0 {
            None
        } else {
            Some(op)
        }
    }

    fn bxc(&mut self, _: u128) {
        self.b = self.b ^ self.c;
    }

    fn out(&mut self, op: u128) {
        self.output.push(self.combo_value(op) % 8)
    }

    fn bdv(&mut self, op: u128) {
        self.b = self.a >> self.combo_value(op);
    }

    fn cdv(&mut self, op: u128) {
        self.c = self.a >> self.combo_value(op);
    }

    fn find_values_outputting(&mut self, target: &[u128], options: &[u128]) -> Vec<u128> {
        let mut values = vec![];

        for option in options {
            for low_bits in 0..2u128.pow(3) {
                self.reset();
                let option = (option << 3) | low_bits;
                self.a = option;
                self.run();
                if &self.output[self.output.len() - target.len()..] == target {
                    values.push(option)
                }
            }
        }

        values
    }
}

fn parse_input(input: &str) -> Computer {
    let mut lines = input.lines();

    let a = register_value(lines.next().unwrap());
    let b = register_value(lines.next().unwrap());
    let c = register_value(lines.next().unwrap());

    lines.next();

    let instructions = lines
        .next()
        .unwrap()
        .replace("Program: ", "")
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    Computer {
        a,
        b,
        c,
        instructions,
        ip: 0,
        output: vec![],
    }
}

fn register_value(line: &str) -> u128 {
    line.trim().split_once(": ").unwrap().1.parse().unwrap()
}
