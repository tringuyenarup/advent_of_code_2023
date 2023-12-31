use aoc_2023_lib::main;

use std::{collections::HashMap, error::Error, str::FromStr};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

main! {
    let input = include_str!("../../inputs/day-5.txt");
    (part_1(input).unwrap(),part_2(input).unwrap())
}

fn part_1(input: &str) -> Result<usize> {
    let mut almanac: Almanac = input.parse()?;
    almanac.plant_seeds();
    Ok(*almanac.relationships["location"].iter().min().unwrap())
}

fn part_2(input: &str) -> Result<usize> {
    let mut almanac: Almanac = input.parse()?;
    Ok(almanac.plant_seeds_with_ranges())
}

#[derive(Debug)]
struct Description {
    rules: Vec<(usize, usize, usize)>,
}
#[derive(Debug)]
struct Almanac {
    relationships: HashMap<String, Vec<usize>>,
    maps: Vec<(String, String, Description)>,
}

impl Almanac {
    fn plant_seeds(&mut self) {
        for (source, destination, description) in self.maps.iter() {
            let mut relationship = Vec::new();
            let mut is_mapped = Vec::new();
            // try to map
            for &(destination_start, source_start, range) in description.rules.iter() {
                for number in self.relationships[source].iter() {
                    if source_start <= *number && *number < source_start + range {
                        relationship.push(*number - source_start + destination_start);
                        is_mapped.push(*number);
                    }
                }
            }
            // fill the non map
            for number in self.relationships[source].iter() {
                if !is_mapped.contains(number) {
                    relationship.push(*number);
                }
            }
            self.relationships
                .insert(String::from(destination), relationship);
        }
    }

    fn plant_seeds_with_ranges(&mut self) -> usize {
        // fix the seed ranges
        let seeds = self.relationships["seed"]
            .chunks(2)
            .map(|w| (w[0], w[0] + w[1]))
            .collect::<Vec<_>>();

        // perform
        self.maps
            .iter()
            .fold(seeds, |seeds, (_, _, mappings)| {
                seeds
                    .iter()
                    .flat_map(|&(start, len)| {
                        let mut mapped = Vec::new();
                        let mut unmapped = vec![(start, len)];

                        for &(dst, src, len) in mappings.rules.iter() {
                            let mut tmp_unmapped = Vec::new();
                            for (start, end) in unmapped {
                                // unmapped on the right
                                let unmapped_right = (start, end.min(src));
                                // accepted range
                                let accepted = (start.max(src), (src + len).min(end));
                                // unmapped on the left
                                let unmapped_left = ((src + len).max(start), end);

                                // add range if valid
                                // all of these cases
                                // can happen simultaneously
                                if accepted.1 > accepted.0 {
                                    mapped.push((accepted.0 - src + dst, accepted.1 - src + dst));
                                }
                                if unmapped_right.1 > unmapped_right.0 {
                                    tmp_unmapped.push(unmapped_right);
                                }
                                if unmapped_left.1 > unmapped_left.0 {
                                    tmp_unmapped.push(unmapped_left);
                                }
                            }
                            unmapped = tmp_unmapped;
                        }
                        // what didn't map remain the same in the next iteration
                        mapped.extend(unmapped);
                        mapped
                    })
                    .collect::<Vec<_>>()
            })
            .iter()
            .map(|&(s, _)| s)
            .min()
            .unwrap()
    }
}

impl FromStr for Almanac {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let mut data = input.split("\n\n");
        let mut relationships = HashMap::new();

        relationships.insert(
            String::from("seed"),
            data.next()
                .unwrap()
                .split_ascii_whitespace()
                .skip(1)
                .map(|seed| seed.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
        let maps = data
            .map(|map| {
                let (content, remain) = map.split_once('\n').unwrap();
                let (destination, source) = content
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .split_once("-to-")
                    .unwrap();
                (
                    String::from(destination),
                    String::from(source),
                    remain.parse::<Description>().unwrap(),
                )
            })
            .collect::<Vec<(String, String, Description)>>();

        Ok(Almanac {
            relationships,
            maps,
        })
    }
}

impl FromStr for Description {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Description {
            rules: s
                .lines()
                .map(|line| {
                    let out = line
                        .split_ascii_whitespace()
                        .map(|number| number.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    (out[0], out[1], out[2])
                })
                .collect::<Vec<_>>(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/day-5-test.txt");
    #[test]
    fn test_input() {
        assert_eq!(part_1(TEST_INPUT).unwrap(), 35);
        assert_eq!(part_2(TEST_INPUT).unwrap(), 46);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(include_str!("../../inputs/day-5.txt")).unwrap(),
            107_430_936
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../../inputs/day-5.txt")).unwrap(),
            23_738_616
        );
    }
}
