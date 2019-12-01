mod day01;
mod util;

use crate::day01::*;

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

    let args = util::get_arguments()?;
    if args.part.is_some() {
        let solution = problems[args.day - 1][args.part.unwrap()]()?;
        println!("Solution {}: {}", args, solution);
    } else {
        let solution_a = problems[args.day - 1][0]()?;
        println!("Solution {}a: {}", args.day, solution_a);
        let solution_b = problems[args.day - 1][1]()?;
        println!("Solution {}b: {}", args.day, solution_b);
    }

    Ok(())
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
