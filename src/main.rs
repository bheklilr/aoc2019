#![feature(test)]
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
    let problems = [[problem01_a, problem01_b]];

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
                .default_value("a")
                .index(2),
        )
        .get_matches();

    let problem = matches
        .value_of("PROBLEM")
        .unwrap()
        .parse::<usize>()
        .map_err(|err| err.to_string())?;
    let part = matches.value_of("PART").unwrap_or("a");
    let part_idx = if part == "a" { 0 } else { 1 };

    let solution = problems[problem - 1][part_idx]()?;
    println!("Solution: {}", solution);
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

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_problem01_a(b: &mut Bencher) {
        b.iter(|| problem01_a())
    }

    #[bench]
    fn bench_problem01_b(b: &mut Bencher) {
        b.iter(|| problem01_b())
    }
}
