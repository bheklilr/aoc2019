use crate::util::lines;
use std::collections::HashMap;

fn load_orbits() -> Result<HashMap<String, Vec<String>>, String> {
    let mut map = HashMap::new();
    for line in lines("inputs/06.txt")?.iter() {
        let bodies: Vec<_> = line.split(',').collect();
        let center = bodies[0].to_string();
        let satellite = bodies[1].to_string();

        let mut empty = vec![];
        let mut existing = map.get_mut(&center).unwrap_or(&mut empty);
        existing.push(satellite);
        map.insert(center, existing.to_vec());
    }
    Ok(map)
}

pub fn day06_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

pub fn day06_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}
