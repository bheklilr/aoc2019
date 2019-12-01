mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod util;

use day01::*;
use day02::*;
use day03::*;
use day04::*;
use day05::*;
use day06::*;
use day07::*;
use day08::*;
use day09::*;
use day10::*;
use day11::*;
use day12::*;
use day13::*;
use day14::*;
use day15::*;
use day16::*;
use day17::*;
use day18::*;
use day19::*;
use day20::*;
use day21::*;
use day22::*;
use day23::*;
use day24::*;
use day25::*;

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
