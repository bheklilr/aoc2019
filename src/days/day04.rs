use itertools::Itertools;

const START: i32 = 165432;
const STOP: i32 = 707912;

fn has_adjacent_matching_digits(i: i32) -> bool {
    i.to_string()
        .chars()
        .zip(i.to_string()[1..].chars())
        .find(|(x, y)| x == y)
        .is_some()
}

fn has_monotonically_increasing_digits(i: i32) -> bool {
    i.to_string()
        .chars()
        .zip(i.to_string()[1..].chars())
        .all(|(x, y)| x <= y)
}

pub fn day04_a() -> Result<String, String> {
    Ok((START..STOP)
        .filter(|&i| i.to_string().len() == 6)
        .filter(|&i| has_adjacent_matching_digits(i))
        .filter(|&i| has_monotonically_increasing_digits(i))
        .count()
        .to_string())
}

fn last_rule(i: i32) -> bool {
    let group_sizes = i
        .to_string()
        .chars()
        .filter_map(|d| d.to_digit(10))
        .group_by(|&i| i)
        .into_iter()
        .map(|group| group.1.count())
        .collect::<Vec<_>>();
    group_sizes.contains(&2)
}

fn solve_b(i: i32) -> bool {
    (i.to_string().len() == 6)
        && (has_adjacent_matching_digits(i))
        && (has_monotonically_increasing_digits(i))
        && (last_rule(i))
}

pub fn day04_b() -> Result<String, String> {
    Ok((START..STOP).filter(|&i| solve_b(i)).count().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_day4_b() {
        assert!(solve_b(112233));
        assert!(!solve_b(123444));
        assert!(solve_b(111122));
    }
}
