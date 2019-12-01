use crate::util::parsed_lines;

pub struct Module {
    pub mass: u64,
}

impl Module {
    pub fn new(mass: u64) -> Module {
        Module { mass: mass }
    }

    pub fn fuel_for(value: &u64) -> Option<u64> {
        value.checked_div(3)?.checked_sub(2)
    }

    pub fn fuel(&self) -> Option<u64> {
        Module::fuel_for(&self.mass)
    }
}

impl Iterator for Module {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let f = self.fuel()?;
        self.mass = f;
        Some(f)
    }
}

fn solve_a(values: &Vec<u64>) -> u64 {
    values
        .iter()
        .filter_map(|value| Module::fuel_for(value))
        .sum()
}

pub fn day01_a() -> Result<String, String> {
    Ok(solve_a(&parsed_lines("inputs/01.txt")?).to_string())
}

fn solve_b(values: &Vec<u64>) -> u64 {
    values
        .iter()
        .flat_map(|value| Module::new(*value).into_iter())
        .sum()
}

pub fn day01_b() -> Result<String, String> {
    Ok(solve_b(&parsed_lines("inputs/01.txt")?).to_string())
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solve_a() -> Result<(), String> {
        assert_eq!(solve_a(&vec![0, 1, 2, 3]), 0);
        assert_eq!(solve_a(&vec![12]), 2);
        assert_eq!(solve_a(&vec![14]), 2);
        assert_eq!(solve_a(&vec![1969]), 654);
        assert_eq!(solve_a(&vec![100756]), 33583);
        assert_eq!(solve_a(&parsed_lines("inputs/01.txt")?), 3325347);
        Ok(())
    }

    #[test]
    fn test_solve_b() -> Result<(), String> {
        assert_eq!(solve_b(&vec![0, 1, 2, 3]), 0);
        assert_eq!(solve_b(&vec![14]), 2);
        assert_eq!(solve_b(&vec![1969]), 966);
        assert_eq!(solve_b(&vec![100756]), 50346);
        assert_eq!(solve_b(&parsed_lines("inputs/01.txt")?), 4985145);
        Ok(())
    }

    #[bench]
    fn bench_solve_a(b: &mut Bencher) -> Result<(), String> {
        let values = parsed_lines("inputs/01.txt")?;
        b.iter(|| solve_a(&values));
        Ok(())
    }

    #[bench]
    fn bench_solve_b(b: &mut Bencher) -> Result<(), String> {
        let values = parsed_lines("inputs/01.txt")?;
        b.iter(|| solve_b(&values));
        Ok(())
    }

    #[bench]
    fn bench_day01_a(b: &mut Bencher) {
        b.iter(day01_a)
    }

    #[bench]
    fn bench_day01_b(b: &mut Bencher) {
        b.iter(day01_b)
    }
}
