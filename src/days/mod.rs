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

pub fn all_days() -> [[fn() -> std::result::Result<std::string::String, std::string::String>; 2]; 25]
{
    [
        [day01::day01_a, day01::day01_b],
        [day02::day02_a, day02::day02_b],
        [day03::day03_a, day03::day03_b],
        [day04::day04_a, day04::day04_b],
        [day05::day05_a, day05::day05_b],
        [day06::day06_a, day06::day06_b],
        [day07::day07_a, day07::day07_b],
        [day08::day08_a, day08::day08_b],
        [day09::day09_a, day09::day09_b],
        [day10::day10_a, day10::day10_b],
        [day11::day11_a, day11::day11_b],
        [day12::day12_a, day12::day12_b],
        [day13::day13_a, day13::day13_b],
        [day14::day14_a, day14::day14_b],
        [day15::day15_a, day15::day15_b],
        [day16::day16_a, day16::day16_b],
        [day17::day17_a, day17::day17_b],
        [day18::day18_a, day18::day18_b],
        [day19::day19_a, day19::day19_b],
        [day20::day20_a, day20::day20_b],
        [day21::day21_a, day21::day21_b],
        [day22::day22_a, day22::day22_b],
        [day23::day23_a, day23::day23_b],
        [day24::day24_a, day24::day24_b],
        [day25::day25_a, day25::day25_b],
    ]
}
