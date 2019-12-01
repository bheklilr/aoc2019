use crate::util::int_lines;

pub struct Module {
    pub mass: i64,
}

impl Module {
    pub fn new(mass: i64) -> Module {
        Module { mass: mass }
    }

    pub fn fuel(&self) -> i64 {
        Module::fuel_for(self.mass)
    }

    pub fn fuel_for(mass: i64) -> i64 {
        std::cmp::max(0, mass / 3 - 2)
    }
}

impl Iterator for Module {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let f = self.fuel();
        self.mass = f;
        if f > 0 {
            Some(f)
        } else {
            None
        }
    }
}

fn solve_a(values: &Vec<i64>) -> i64 {
    values.iter().map(|value| Module::fuel_for(*value)).sum()
}

pub fn day01_a() -> Result<String, String> {
    Ok(solve_a(&int_lines("inputs/01.txt")?).to_string())
}

fn solve_b(values: &Vec<i64>) -> i64 {
    values
        .iter()
        .flat_map(|value| Module::new(*value).into_iter())
        .sum()
}

pub fn day01_b() -> Result<String, String> {
    Ok(solve_b(&int_lines("inputs/01.txt")?).to_string())
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
        assert_eq!(solve_a(&int_lines("inputs/01.txt")?), 3325347);
        Ok(())
    }

    #[test]
    fn test_solve_b() -> Result<(), String> {
        assert_eq!(solve_b(&vec![0, 1, 2, 3]), 0);
        assert_eq!(solve_b(&vec![14]), 2);
        assert_eq!(solve_b(&vec![1969]), 966);
        assert_eq!(solve_b(&vec![100756]), 50346);
        assert_eq!(solve_b(&int_lines("inputs/01.txt")?), 4985145);
        Ok(())
    }

    #[bench]
    fn bench_solve_a(b: &mut Bencher) -> Result<(), String> {
        let values = int_lines("inputs/01.txt")?;
        b.iter(|| solve_a(&values));
        Ok(())
    }

    #[bench]
    fn bench_solve_b(b: &mut Bencher) -> Result<(), String> {
        let values = int_lines("inputs/01.txt")?;
        b.iter(|| solve_b(&values));
        Ok(())
    }
}
