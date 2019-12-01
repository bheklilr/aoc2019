use crate::util::int_lines;

pub fn problem01_a() -> Result<String, String> {
    let total = int_lines("inputs/01.txt")?
        .iter()
        .map(|value| value / 3 - 2)
        .sum::<i64>();
    Ok(total.to_string())
}

pub fn problem01_b() -> Result<String, String> {
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
