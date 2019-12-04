use crate::util::*;
use std::cmp::{max, min};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    direction: Direction,
    distance: i64,
}

fn parse_int(s: &str) -> Result<i64, String> {
    Ok(s.parse()
        .map_err(|e: std::num::ParseIntError| e.to_string())?)
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.chars();
        match cs.next() {
            Some('U') => Ok(Move::new(Direction::U, parse_int(&s[1..])?)),
            Some('D') => Ok(Move::new(Direction::D, parse_int(&s[1..])?)),
            Some('L') => Ok(Move::new(Direction::L, parse_int(&s[1..])?)),
            Some('R') => Ok(Move::new(Direction::R, parse_int(&s[1..])?)),
            _ => Err("Bad move".to_string()),
        }
    }
}

impl Move {
    pub fn new(dir: Direction, dist: i64) -> Move {
        Move {
            direction: dir,
            distance: dist,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point(i64, i64);

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Point {
    pub fn magnitude(&self) -> i64 {
        self.0.abs() + self.1.abs()
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    pub start: Point,
    pub end: Point,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.start, self.end)
    }
}

impl Line {
    fn horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn as_ascending(&self) -> Line {
        if self.horizontal() {
            Line {
                start: Point(min(self.start.0, self.end.0), self.start.1),
                end: Point(max(self.start.0, self.end.0), self.start.1),
            }
        } else {
            Line {
                start: Point(self.start.0, min(self.start.1, self.end.1)),
                end: Point(self.start.0, max(self.start.1, self.end.1)),
            }
        }
    }

    fn horizontal_then_vertical(a: &Line, b: &Line) -> Option<(Line, Line)> {
        if a.horizontal() & b.horizontal() {
            return None;
        }
        if a.vertical() & b.vertical() {
            return None;
        }
        if a.horizontal() {
            Some((a.as_ascending(), b.as_ascending()))
        } else {
            Some((b.as_ascending(), a.as_ascending()))
        }
    }

    pub fn intersection(&self, other: &Line) -> Option<Point> {
        let (a, b) = Line::horizontal_then_vertical(self, other)?;

        let intersects_horizontally = (a.start.0 <= b.start.0) & (a.end.0 >= b.end.0);
        let intersects_vertically = (b.start.1 <= a.start.1) & (b.end.1 >= a.end.1);
        let intersects = intersects_horizontally & intersects_vertically;
        if intersects {
            Some(Point(b.start.0, a.start.1))
        } else {
            None
        }
    }
}

fn move_point(point: &Point, movement: &Move) -> Point {
    match movement.direction {
        Direction::U => Point(point.0, point.1 + movement.distance),
        Direction::D => Point(point.0, point.1 - movement.distance),
        Direction::L => Point(point.0 - movement.distance, point.1),
        Direction::R => Point(point.0 + movement.distance, point.1),
    }
}

fn full_path(movements: &Vec<Move>) -> Vec<Line> {
    movements
        .iter()
        .scan(Point(0, 0), |loc, next_move| {
            let new_loc = move_point(loc, &next_move);
            let path = Line {
                start: Point(loc.0, loc.1),
                end: new_loc,
            };
            loc.0 = new_loc.0;
            loc.1 = new_loc.1;
            Some(path)
        })
        .collect()
}

fn moves() -> Result<Vec<Vec<Move>>, String> {
    let input: Vec<Vec<Move>> = lines("inputs/03.txt")?
        .iter()
        .map(|line| {
            line.split(',')
                .map(|v| v.parse())
                .filter_map(|v| v.ok())
                .collect()
        })
        .collect();
    Ok(input)
}

fn solve_a(wire1: &Vec<Move>, wire2: &Vec<Move>) -> Result<i64, String> {
    let wire1_path = full_path(wire1);
    let wire2_path = full_path(wire2);
    wire1_path
        .iter()
        .flat_map(|p1| wire2_path.iter().filter_map(move |p2| p1.intersection(p2)))
        .map(|p| p.magnitude())
        .filter(|p| *p != 0)
        .min()
        .ok_or("No intersections found".to_string())
}

pub fn day03_a() -> Result<String, String> {
    let input = moves()?;
    let distance = solve_a(&input[0], &input[1]);
    Ok(distance?.to_string())
}

fn full_path_counting(movements: &Vec<Move>) -> Vec<(Line, i64)> {
    movements
        .iter()
        .scan((Point(0, 0), 0), |state, next_move| {
            let new_loc = move_point(&state.0, &next_move);
            let path = Line {
                start: Point((state.0).0, (state.0).1),
                end: new_loc,
            };
            (state.0).0 = new_loc.0;
            (state.0).1 = new_loc.1;
            state.1 += next_move.distance;
            Some((path, state.1 - next_move.distance))
        })
        .collect()
}

fn manhattan(a: Point, b: Point) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn solve_b(wire1: &Vec<Move>, wire2: &Vec<Move>) -> Result<i64, String> {
    // The tricky bit is that you have to include the "partial steps"
    // between the last locations before the intersection and the
    // intersection itself
    let wire1_path = full_path_counting(wire1);
    let wire2_path = full_path_counting(wire2);
    wire1_path
        .iter()
        .flat_map(|(p1, p1_steps)| {
            wire2_path
                .iter()
                .map(move |(p2, p2_steps)| (p1, p2, p1_steps, p2_steps))
        })
        .filter_map(|(p1, p2, p1_steps, p2_steps)| {
            p1.intersection(p2)
                .map(|int| (p1, p2, p1_steps, p2_steps, int))
        })
        .map(|(p1, p2, p1_steps, p2_steps, int)| {
            p1_steps + p2_steps + manhattan(p1.start, int) + manhattan(p2.start, int)
        })
        .min()
        .ok_or("No intersections found".to_string())
}

pub fn day03_b() -> Result<String, String> {
    let input = moves()?;
    let solution = solve_b(&input[0], &input[1])?;
    Ok(solution.to_string())
}
