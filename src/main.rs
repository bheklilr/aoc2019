use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

fn is_numeric(value: String) -> Result<(), String> {
    value
        .parse::<i32>()
        .map(|_parsed| ())
        .map_err(|err| err.to_string())
}

fn main() -> Result<(), String> {
    let problems = [
        [problem01_a, problem01_b],
        [problem02_a, problem02_b],
        [problem03_a, problem03_b],
        [problem04_a, problem04_b],
        [problem05_a, problem05_b],
        [problem06_a, problem06_b],
        [problem07_a, problem07_b],
        [problem08_a, problem08_b],
        [problem09_a, problem09_b],
        [problem10_a, problem10_b],
        [problem11_a, problem11_b],
        [problem12_a, problem12_b],
        [problem13_a, problem13_b],
        [problem14_a, problem14_b],
        [problem15_a, problem15_b],
        [problem16_a, problem16_b],
        [problem17_a, problem17_b],
        [problem18_a, problem18_b],
        [problem19_a, problem19_b],
        [problem20_a, problem20_b],
        [problem21_a, problem21_b],
        [problem22_a, problem22_b],
        [problem23_a, problem23_b],
        [problem24_a, problem24_b],
        [problem25_a, problem25_b],
    ];

    let matches = App::new("aoc2019")
        .version("1.0")
        .author("Aaron Stevens")
        .about("Advent of Code 2019 solution runner")
        .arg(
            Arg::with_name("PROBLEM")
                .help("The problem number to run")
                .required(true)
                .index(1)
                .validator(is_numeric),
        )
        .arg(
            Arg::with_name("PART")
                .help("The problem part to run")
                .possible_values(&["a", "b"])
                .required(false)
                .index(2),
        )
        .get_matches();

    let problem = matches
        .value_of("PROBLEM")
        .unwrap()
        .parse::<usize>()
        .map_err(|err| err.to_string())?;
    let part = matches.value_of("PART");
    if part.is_some() {
        let part_idx = if part.unwrap() == "a" { 0 } else { 1 };
        let solution = problems[problem - 1][part_idx]()?;
        println!("Solution {}{}: {}", problem, part.unwrap(), solution);
    } else {
        let solution_a = problems[problem - 1][0]()?;
        println!("Solution {}a: {}", problem, solution_a);
        let solution_b = problems[problem - 1][1]()?;
        println!("Solution {}b: {}", problem, solution_b);
    }

    Ok(())
}

fn lines(filename: &str) -> Result<Vec<String>, String> {
    let file = File::open(filename).map_err(|err| err.to_string())?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.len() > 0)
        .collect())
}

fn int_lines(filename: &str) -> Result<Vec<i64>, String> {
    Ok(lines(filename)?
        .iter()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect())
}

fn problem01_a() -> Result<String, String> {
    let total = int_lines("inputs/01.txt")?
        .iter()
        .map(|value| value / 3 - 2)
        .sum::<i64>();
    Ok(total.to_string())
}

fn problem01_b() -> Result<String, String> {
    let total = int_lines("inputs/01.txt")?
        .iter()
        .map(|value| {
            let mut module_total = 0;
            let mut fuel = value / 3 - 2;
            while fuel > 0 {
                module_total += fuel;
                fuel = fuel / 3 - 2;
            }
            module_total
        })
        .sum::<i64>();
    Ok(total.to_string())
}

fn problem02_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem02_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem03_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem03_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem04_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem04_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem05_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem05_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem06_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem06_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem07_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem07_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem08_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem08_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem09_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem09_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem10_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem10_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem11_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem11_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem12_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem12_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem13_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem13_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem14_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem14_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem15_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem15_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem16_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem16_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem17_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem17_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem18_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem18_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem19_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem19_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem20_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem20_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem21_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem21_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem22_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem22_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem23_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem23_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem24_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem24_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem25_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

fn problem25_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}
