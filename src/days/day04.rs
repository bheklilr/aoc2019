const START: i32 = 165432;
const STOP: i32 = 707912;

fn has_adjacent_matching_digits(i: i32) -> bool {
    i.to_string().chars().zip(i.to_string()[1..].chars())
        .filter(|(x, y)| x == y)
        .collect::<Vec<_>>()
        .len() > 0
}

fn has_monotonically_increasing_digits(i: i32) -> bool {
    i.to_string().chars().zip(i.to_string()[1..].chars())
        .all(|(x, y)| x <= y)
}

pub fn day04_a() -> Result<String, String> {
    Ok((START..STOP)
        .filter(|&i| i.to_string().len() == 6)
        .filter(|&i| has_adjacent_matching_digits(i))
        .filter(|&i| has_monotonically_increasing_digits(i))
        .collect::<Vec<_>>()
        .len()
        .to_string())
}

pub fn day04_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}
