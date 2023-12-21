use aoc_2023_lib::{main, utils::lcm};

use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    str::FromStr,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-20.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let mut configuration: Configuration = input.parse()?;
    let mut ans = [0; 2];
    for _ in 0..1_000 {
        let (a, b) = configuration.generate_pulse(&[]);
        ans[0] += a;
        ans[1] += b;
    }
    Ok(ans.iter().product())
}

fn part_2(input: &str) -> Result<usize> {
    let mut configuration: Configuration = input.parse()?;
    // rx is flipflop and it has 1 predecessor - bq which is an invertor
    // therefore, if rx wants to receive a LOW -> invertor inputs must all be
    // HIGH
    // SOLUTION:
    // all of predecessor of rx's predecessor must give a HIGH inputs

    let source = configuration
        .modules
        .iter()
        .find(|m| m.1.outputs.contains(&String::from("rx")))
        .expect("ERROR: must be a module lead to rx")
        .0;
    let mut predecessors = Vec::new();
    for (name, Module { outputs, .. }) in configuration.modules.iter() {
        if outputs.contains(source) {
            predecessors.push(name.clone());
        }
    }

    let mut cycles = vec![0usize; predecessors.len()];
    let mut count = 0;
    loop {
        count += 1;
        if let (0, c) = configuration.generate_pulse(&predecessors) {
            cycles[c] = count;
            if cycles.iter().all(|c| *c != 0) {
                break;
            }
        }
    }
    Ok(cycles.iter().fold(1, |mut acc, c| {
        acc = lcm(acc, *c);
        acc
    }))
}

#[derive(Debug)]
struct Configuration {
    modules: HashMap<String, Module>,
}

impl Configuration {
    fn generate_pulse(&mut self, predecessors: &[String]) -> (usize, usize) {
        // start pressing the button with low signal
        let mut queue =
            VecDeque::from([(String::from("button"), String::from("broadcaster"), false)]);
        let mut count = [0; 2];
        let mut cycle = 0;
        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            if receiver == "rx" && !pulse {
                return (0, 0);
            }
            count[pulse as usize] += 1;
            if let Some(Module {
                module_type,
                outputs,
            }) = self.modules.get_mut(&receiver)
            {
                match module_type {
                    ModuleType::FlipFlop(flipflop) => {
                        if !pulse {
                            *flipflop = !*flipflop;
                            for o in outputs {
                                queue.push_back((receiver.clone(), o.clone(), *flipflop));
                            }
                        }
                    }
                    ModuleType::Conjunction(conjunction) => {
                        conjunction.insert(sender.clone(), pulse);
                        let out = !conjunction.values().all(|v| *v);
                        if !predecessors.is_empty() && out {
                            if let Some(predecessor) =
                                predecessors.iter().position(|p| *p == receiver)
                            {
                                cycle = predecessor + 1;
                            }
                        }

                        for o in outputs {
                            queue.push_back((receiver.clone(), o.clone(), out));
                        }
                    }
                    ModuleType::Broadcaster => {
                        for o in outputs {
                            queue.push_back((receiver.clone(), o.clone(), false));
                        }
                    }
                }
            }
        }
        if cycle != 0 {
            (0, cycle - 1)
        } else {
            (count[0], count[1])
        }
    }
}

impl FromStr for Configuration {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Configuration> {
        let mut conjunctions = HashMap::<String, HashMap<String, bool>>::new();
        let mut modules = HashMap::new();
        for line in input.lines() {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();
            let rhs = rhs.split(", ").map(|s| s.to_string()).collect();

            if let Some(name) = lhs.strip_prefix('%') {
                modules.insert(
                    name.to_string(),
                    Module {
                        module_type: ModuleType::FlipFlop(false),
                        outputs: rhs,
                    },
                );
            } else if let Some(name) = lhs.strip_prefix('&') {
                conjunctions.insert(name.to_string(), HashMap::new());
                modules.insert(
                    name.to_string(),
                    Module {
                        module_type: ModuleType::Conjunction(HashMap::new()),
                        outputs: rhs,
                    },
                );
            } else {
                modules.insert(
                    lhs.to_string(),
                    Module {
                        module_type: ModuleType::Broadcaster,
                        outputs: rhs,
                    },
                );
            }
        }

        for (name, module) in &modules {
            for out in &module.outputs {
                if let Some(conj) = conjunctions.get_mut(out) {
                    conj.insert(name.clone(), false);
                }
            }
        }

        for (name, module) in modules.iter_mut() {
            if let Some(conj) = conjunctions.get(name) {
                module.module_type = ModuleType::Conjunction(conj.clone());
            }
        }

        Ok(Self { modules })
    }
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    outputs: Vec<String>,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_1: &str = include_str!("../../inputs/day-20-test-1.txt");
    const TEST_INPUT_2: &str = include_str!("../../inputs/day-20-test-2.txt");

    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT_1).unwrap(), 32_000_000);
        assert_eq!(part_1(TEST_INPUT_2).unwrap(), 11_687_500);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-20.txt")).unwrap(),
            681_194_780
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-20.txt")).unwrap(),
            238_593_356_738_827
        );
    }
}
