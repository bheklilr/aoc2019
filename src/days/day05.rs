use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const HALT: i32 = 99;
const ADD: i32 = 1;
const MUL: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const JMP_TRUE: i32 = 5;
const JMP_FALSE: i32 = 6;
const LT: i32 = 7;
const EQ: i32 = 8;

type MResult<T> = Result<T, String>;

pub struct Machine<'a> {
    pub program: &'a mut [i32],
    pointer: usize,
    output: i32,
}

impl<'a> Machine<'a> {
    pub fn new(ops: &'a mut [i32]) -> Machine<'a> {
        Machine {
            program: ops,
            pointer: 0,
            output: -1,
        }
    }

    fn parse_op(&mut self) -> (i32, bool, bool) {
        let instr = self.program[self.pointer];
        let op = instr % 100;
        let modes = (instr - op) / 100;
        let m1 = (modes / 1) % 10 > 0;
        let m2 = (modes / 10) % 10 > 0;
        // let m3 = (modes / 100) % 10 > 0;
        (op, m1, m2)
    }

    fn get_arg(&mut self, index: usize, immediate: bool) -> i32 {
        let reg = self.program[self.pointer + index];
        if immediate {
            reg
        } else {
            self.program[reg as usize]
        }
    }

    fn debug(&mut self, values: usize) {
        let (op, m1, m2) = self.parse_op();
        print!("{:05}:\t", self.pointer);
        print!("({:05})\t", self.program[self.pointer]);
        print!(
            "{}",
            match op {
                HALT => "HALT:      ",
                ADD => "ADD:       ",
                MUL => "MUL:       ",
                INPUT => "INPUT:     ",
                OUTPUT => "OUTPUT:    ",
                JMP_TRUE => "JMP_TRUE:  ",
                JMP_FALSE => "JMP_FALSE: ",
                LT => "LT:        ",
                EQ => "EQ:        ",
                _ => "",
            }
        );
        for i in 1..values {
            print!("\t");
            match i {
                1 => print!("{}", if m1 { " " } else { "$" }),
                2 => print!("{}", if m2 { " " } else { "$" }),
                // 3 => print!("{}", if m3 { " " } else { "$" }),
                _ => {}
            }
            print!("{:05}", self.program[self.pointer + i]);
            match i {
                1 => {
                    if !m1 {
                        print!(
                            "=({:05})",
                            self.program[self.program[self.pointer + i] as usize]
                        )
                    } else {
                        print!("        ")
                    }
                }
                2 => {
                    if !m2 {
                        print!(
                            "=({:05})",
                            self.program[self.program[self.pointer + i] as usize]
                        )
                    } else {
                        print!("        ")
                    }
                }
                // 3 => {
                //     if !m3 {
                //         print!(
                //             "=({:05})",
                //             self.program[self.program[self.pointer + i] as usize]
                //         )
                //     } else {
                //         print!("        ")
                //     }
                // }
                _ => {}
            }
        }
        println!();
    }

    fn handle_halt(&mut self) -> MResult<i32> {
        // self.debug(1);
        Ok(self.output)
    }

    fn handle_add(&mut self, r1_immediate: bool, r2_immediate: bool) -> MResult<()> {
        // self.debug(4);
        let r1 = self.get_arg(1, r1_immediate);
        let r2 = self.get_arg(2, r2_immediate);
        let r3 = self.get_arg(3, true) as usize;
        self.program[r3] = r1 + r2;
        self.pointer += 4;
        Ok(())
    }

    fn handle_mul(&mut self, r1_immediate: bool, r2_immediate: bool) -> MResult<()> {
        // self.debug(4);
        let r1 = self.get_arg(1, r1_immediate);
        let r2 = self.get_arg(2, r2_immediate);
        let r3 = self.get_arg(3, true) as usize;
        self.program[r3] = r1 * r2;
        self.pointer += 4;
        Ok(())
    }

    fn handle_input(&mut self, input: i32) -> MResult<()> {
        // self.debug(2);
        let r = self.get_arg(1, true) as usize; // Not actually immediate, but can be treated so
        self.program[r] = input;
        self.pointer += 2;
        Ok(())
    }

    fn handle_output(&mut self, immediate: bool) -> MResult<i32> {
        // self.debug(2);
        let r = self.get_arg(1, immediate);
        self.pointer += 2;
        Ok(r)
    }

    fn handle_jmp_true(&mut self, r1_immediate: bool, r2_immediate: bool) -> MResult<()> {
        // self.debug(3);
        let x1 = self.get_arg(1, r1_immediate);
        let x2 = self.get_arg(2, r2_immediate);
        if x1 != 0 {
            self.pointer = x2 as usize;
        } else {
            self.pointer += 3;
        }
        Ok(())
    }

    fn handle_jmp_false(&mut self, r1_immediate: bool, r2_immediate: bool) -> MResult<()> {
        // self.debug(3);
        let x1 = self.get_arg(1, r1_immediate);
        let x2 = self.get_arg(2, r2_immediate);
        if x1 == 0 {
            self.pointer = x2 as usize;
        } else {
            self.pointer += 3;
        }
        Ok(())
    }

    fn handle_lt(&mut self, r1_immediate: bool, r2_immediate: bool) -> MResult<()> {
        // self.debug(4);
        let x1 = self.get_arg(1, r1_immediate);
        let x2 = self.get_arg(2, r2_immediate);
        let x3 = self.get_arg(3, true);
        if x1 < x2 {
            self.program[x3 as usize] = 1;
        } else {
            self.program[x3 as usize] = 0;
        }
        self.pointer += 4;
        Ok(())
    }

    fn handle_eq(&mut self, r1_immediate: bool, r2_immediate: bool) -> MResult<()> {
        // self.debug(4);
        let x1 = self.get_arg(1, r1_immediate);
        let x2 = self.get_arg(2, r2_immediate);
        let x3 = self.get_arg(3, true);
        if x1 == x2 {
            self.program[x3 as usize] = 1;
        } else {
            self.program[x3 as usize] = 0;
        }
        self.pointer += 4;
        Ok(())
    }

    pub fn evaluate(&mut self, stdin: &'a [i32]) -> MResult<Vec<i32>> {
        let mut inputs = stdin.iter();
        let mut outputs = vec![];
        loop {
            let (op, m1, m2) = self.parse_op();
            match op {
                HALT => {
                    self.handle_halt()?;
                    return Ok(outputs);
                }
                ADD => self.handle_add(m1, m2)?,
                MUL => self.handle_mul(m1, m2)?,
                INPUT => self.handle_input(*inputs.next().ok_or("No inputs left".to_string())?)?,
                OUTPUT => outputs.push(self.handle_output(m1)?),
                JMP_TRUE => self.handle_jmp_true(m1, m2)?,
                JMP_FALSE => self.handle_jmp_false(m1, m2)?,
                LT => self.handle_lt(m1, m2)?,
                EQ => self.handle_eq(m1, m2)?,
                _ => return Err("Unknown op code".to_string()),
            }
        }
    }
}

fn read_input() -> MResult<Vec<i32>> {
    let file = File::open("inputs/05.txt").map_err(|err| err.to_string())?;
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .nth(0)
        .ok_or("Empty file".to_string())
        .map(|text| {
            text.split(',')
                .filter_map(|code| code.parse().ok())
                .collect()
        })
}

pub fn day05_a() -> MResult<String> {
    let mut input = read_input()?;
    let mut machine = Machine::new(&mut input);
    let output = machine.evaluate(&vec![1])?;
    let solution = output.iter().last().ok_or("Failed to solve".to_string())?;
    Ok(solution.to_string())
}

pub fn day05_b() -> MResult<String> {
    let mut input = read_input()?;
    let mut machine = Machine::new(&mut input);
    let output = machine.evaluate(&vec![5])?;
    let solution = output.iter().nth(0).ok_or("Failed to solve".to_string())?;
    Ok(solution.to_string())
}
