use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let (rules, updates) = parse_input(INPUT);
    let rule_graph = gen_rule_graph(&rules);

    let (compliant, non_compliant) = updates.iter().partition::<Vec<_>, _>(|update| {
        for i in 0..update.len() {
            let elem = update[i];
            let Some(must_after) = rule_graph.get(&elem) else {
                continue;
            };

            for after in &update[i + 1..] {
                if !must_after.contains(after) {
                    return false;
                }
            }
        }

        true
    });

    println!(
        "Part 1: {}",
        compliant.iter().map(|c| c[c.len() / 2]).sum::<usize>()
    );

    println!(
        "Part 2: {}",
        non_compliant
            .iter()
            .map(|c| {
                let fixed = fix_non_compliant(c, &rule_graph);

                fixed[fixed.len() / 2]
            })
            .sum::<usize>()
    );
}

type Rule = (usize, usize);
type Update = Vec<usize>;
type RuleGraph = HashMap<usize, HashSet<usize>>;

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let (rules_src, updates_src) = input.split_once("\n\n").unwrap();

    let rules = rules_src
        .lines()
        .map(|r| {
            let mut elems = r.split("|").map(|i| i.parse::<usize>().unwrap());

            (elems.next().unwrap(), elems.next().unwrap())
        })
        .collect();

    let updates = updates_src
        .lines()
        .map(|line| line.split(',').map(|it| it.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn gen_rule_graph(rules: &[Rule]) -> RuleGraph {
    let mut map = HashMap::<usize, HashSet<usize>>::new();

    for &(before, after) in rules {
        map.entry(before).or_default().insert(after);

        // Ensure that the reverse rule also exists
        map.entry(after).or_default();
    }

    map
}

fn fix_non_compliant(update: &[usize], rule_graph: &RuleGraph) -> Vec<usize> {
    // Yay topological sort

    let mut pending_elems = HashSet::from_iter(update.iter().map(|i| *i));
    let mut ord = Vec::<usize>::new();
    let mut no_after_requirement = pending_elems
        .iter()
        .filter(|it| {
            let Some(afters) = rule_graph.get(it) else {
                return true;
            };
            afters.intersection(&pending_elems).count() == 0
        })
        .map(|it| *it)
        .collect::<Vec<_>>();

    while no_after_requirement.len() > 0 {
        let curr = no_after_requirement.pop().unwrap();
        pending_elems.remove(&curr);
        ord.push(curr);

        no_after_requirement = pending_elems
            .iter()
            .filter(|it| {
                let Some(afters) = rule_graph.get(it) else {
                    return true;
                };
                afters.intersection(&pending_elems).count() == 0
            })
            .map(|it| *it)
            .collect::<Vec<_>>();
    }

    ord.reverse();

    ord
}
