use aoc_2023_lib::main;
use itertools::Itertools;

use std::{collections::HashMap, error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-19.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<i32> {
    let data = input.parse::<WorkFlows>()?;
    let mut ans = 0;
    for system in data.system.iter() {
        let mut start_workflow = "in";
        let mut index = 0;
        loop {
            // know for sure it must be inside the map
            let rules = &data.rules[start_workflow];

            // only take the first rule
            match &rules[index] {
                Rule::Accept => {
                    ans += system.values().sum::<i32>();
                    break;
                }
                Rule::Reject => break,
                Rule::WorkFlow(workflow) => {
                    start_workflow = workflow;
                    index = 0;
                }
                Rule::Condition((s, comparison, value, result)) => {
                    let comparison_result = match comparison {
                        '>' => system[s] > *value,
                        '<' => system[s] < *value,
                        _ => panic!("ERROR: only support > or < operation"),
                    };
                    if comparison_result {
                        if result == "A" {
                            ans += system.values().sum::<i32>();
                            break;
                        } else if result == "R" {
                            break;
                        } else {
                            start_workflow = result;
                            index = 0;
                        }
                    } else {
                        index += 1;
                    }
                }
            }
        }
    }
    Ok(ans)
}
//https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/19.rs
fn part_2(input: &str) -> Result<usize> {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .split('\n')
        .map(|l| {
            let (name, rest) = l.split_once('{').unwrap();
            let (rest, label) = rest[0..rest.len() - 1].split_at(rest.rfind(',').unwrap());
            let rules = rest
                .split(',')
                .map(|rule| {
                    let (rest, label) = rule.split_once(':').unwrap();
                    let op = if rest.contains('<') { '<' } else { '>' };
                    let (name, n) = rest.split_once(op).unwrap();
                    (
                        name.as_bytes()[0] as char,
                        op,
                        n.parse::<usize>().unwrap(),
                        label,
                    )
                })
                .collect::<Vec<_>>();
            (name, (rules, &label[1..]))
        })
        .collect::<HashMap<_, _>>();

    Ok(count_accepted(
        &workflows,
        "in",
        std::array::from_fn(|_| (1..=4000).collect::<Vec<_>>()),
    ))
}

type WorkFlows2<'a> = HashMap<&'a str, (Vec<(char, char, usize, &'a str)>, &'a str)>;

fn count_accepted<'a>(
    workflows: &WorkFlows2<'a>,
    curr: &'a str,
    mut ranges: [Vec<usize>; 4],
) -> usize {
    if curr == "A" {
        return ranges.iter().map(|v| v.len()).product();
    }
    if curr == "R" {
        return 0;
    }
    let mut ans = 0;
    let workflow = &workflows[curr];

    for &(p, op, n, label) in &workflow.0 {
        let i = "xmas".chars().position(|c| c == p).unwrap();
        let (r, tmp) = ranges[i]
            .iter()
            .partition(|&&val| if op == '<' { val < n } else { val > n });
        let mut newranges = ranges.clone();
        newranges[i] = r;
        ans += count_accepted(workflows, label, newranges);
        ranges[i] = tmp;
    }
    ans += count_accepted(workflows, workflow.1, ranges);
    ans
}

impl FromStr for WorkFlows {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let (left, right) = input
            .split_once("\n\n")
            .expect("ERROR: input should contain 1 empty line");

        let rules = left
            .lines()
            .map(|line| {
                let (workflow, rules) = line
                    .split_once('{')
                    .expect("ERROR: each line should contain 1 {");
                (
                    String::from(workflow),
                    rules
                        .trim_end_matches('}')
                        .split(',')
                        .map(|rule| match rule.contains(':') {
                            true => {
                                let (condition, target) = rule.split_once(':').unwrap();
                                let mut condition = condition.chars();

                                Rule::Condition((
                                    condition.next().unwrap(),
                                    condition.next().unwrap(),
                                    condition.collect::<String>().parse().unwrap(),
                                    String::from(target),
                                ))
                            }
                            false => match rule {
                                "A" => Rule::Accept,
                                "R" => Rule::Reject,
                                _ => Rule::WorkFlow(String::from(rule)),
                            },
                        })
                        .collect_vec(),
                )
            })
            .collect::<HashMap<String, Vec<Rule>>>();
        let system = right
            .lines()
            .map(|line| {
                line.trim_end_matches('}')
                    .trim_start_matches('{')
                    .split(',')
                    .map(|p| {
                        let (name, value) = p.split_once('=').unwrap();
                        (name.chars().next().unwrap(), value.parse().unwrap())
                    })
                    .collect::<HashMap<char, i32>>()
            })
            .collect_vec();
        Ok(Self { rules, system })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Rule {
    Accept,
    Reject,
    WorkFlow(String),
    Condition((char, char, i32, String)),
}

#[derive(Debug)]
struct WorkFlows {
    rules: HashMap<String, Vec<Rule>>,
    system: Vec<HashMap<char, i32>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-19-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 19_114);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 167_409_079_868_000);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-19.txt")).unwrap(),
            395_382
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-19.txt")).unwrap(),
            103_557_657_654_583
        );
    }
}
