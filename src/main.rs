use once_cell::sync::Lazy;
use regex::Regex;
use std::process::Command;

static MS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Time: (\d+) ms").unwrap());

fn extract_time(s: &str) -> u32 {
    let capture = MS_REGEX.captures_iter(s).next().unwrap();
    capture[1].parse().unwrap()
}

fn main() {
    let total_time = (1..=25)
        .map(|day_num| {
            let day = format!("day-{}", day_num);
            let cmd = Command::new("cargo")
                .args(["run", "--release", "--bin", &day])
                .output()
                .unwrap();
            let output = String::from_utf8(cmd.stdout).unwrap();
            println!("{}:\n{}", day, output);
            extract_time(&output)
        })
        .sum::<u32>();
    println!("Total time: {}ms", total_time);
}
