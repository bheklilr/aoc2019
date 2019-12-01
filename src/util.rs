use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

pub fn lines(filename: &str) -> Result<Vec<String>, String> {
    let file = File::open(filename).map_err(|err| err.to_string())?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.len() > 0)
        .collect())
}

pub fn int_lines(filename: &str) -> Result<Vec<i64>, String> {
    parsed_lines(filename)
}

pub fn parsed_lines<T: std::str::FromStr>(filename: &str) -> Result<Vec<T>, String> {
    Ok(lines(filename)?
        .iter()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub struct Arguments {
    pub day: usize,
    pub part: Option<usize>,
}

fn is_numeric(value: String) -> Result<(), String> {
    value
        .parse::<i32>()
        .map(|_parsed| ())
        .map_err(|err| err.to_string())
}

pub fn get_arguments() -> Result<Arguments, String> {
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

    Ok(Arguments::new(
        problem,
        part.map(|a_or_b| if a_or_b == "a" { 0 } else { 1 }),
    ))
}

impl Arguments {
    fn new(day: usize, part: Option<usize>) -> Arguments {
        Arguments {
            day: day,
            part: part,
        }
    }
}

impl std::fmt::Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.part.is_none() {
            write!(f, "{}", self.day)
        } else {
            let a_or_b = self.part.map(|p| if p == 0 { "a" } else { "b" }).unwrap();
            write!(f, "{}{}", self.day, a_or_b)
        }
    }
}
