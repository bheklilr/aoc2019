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
    let days = [
        [day01_a, day01_b],
        [day02_a, day02_b],
        [day03_a, day03_b],
        [day04_a, day04_b],
        [day05_a, day05_b],
        [day06_a, day06_b],
        [day07_a, day07_b],
        [day08_a, day08_b],
        [day09_a, day09_b],
        [day10_a, day10_b],
        [day11_a, day11_b],
        [day12_a, day12_b],
        [day13_a, day13_b],
        [day14_a, day14_b],
        [day15_a, day15_b],
        [day16_a, day16_b],
        [day17_a, day17_b],
        [day18_a, day18_b],
        [day19_a, day19_b],
        [day20_a, day20_b],
        [day21_a, day21_b],
        [day22_a, day22_b],
        [day23_a, day23_b],
        [day24_a, day24_b],
        [day25_a, day25_b],
    ];

    let args = util::get_arguments()?;
    if args.part.is_some() {
        let solution = days[args.day - 1][args.part.unwrap()]()?;
        println!("Solution {}: {}", args, solution);
    } else {
        let solution_a = days[args.day - 1][0]()?;
        println!("Solution {}a: {}", args.day, solution_a);
        let solution_b = days[args.day - 1][1]()?;
        println!("Solution {}b: {}", args.day, solution_b);
    }

    Ok(())
}
