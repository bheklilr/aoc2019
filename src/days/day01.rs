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
        mass / 3 - 2
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

pub fn day01_a() -> Result<String, String> {
    let total = int_lines("inputs/01.txt")?
        .iter()
        .map(|value| Module::fuel_for(*value))
        .sum::<i64>();
    Ok(total.to_string())
}

pub fn day01_b() -> Result<String, String> {
    let total = int_lines("inputs/01.txt")?
        .iter()
        .flat_map(|value| Module::new(*value).into_iter())
        .sum::<i64>();
    Ok(total.to_string())
}
