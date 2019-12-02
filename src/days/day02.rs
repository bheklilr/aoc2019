use crate::util::parsed_lines;

pub fn eval(instructions: Vec<usize>) -> usize {
    let mut mem = instructions;

    let mut finished = false;
    let mut pos = 0;
    while !finished && pos < mem.len() {
        if mem[pos] == 99 {
            finished = true;
        } else {
            let x1 = mem[mem[pos + 1]];
            let x2 = mem[mem[pos + 2]];
            let res = mem[pos + 3];
            if mem[pos] == 1 {
                // Add
                mem[res] = x1 + x2;
            } else if mem[pos] == 2 {
                mem[res] = x1 * x2;
            }
            pos += 4;
        }
    }
    mem[0]
}

fn solve_a(mut prog: Vec<usize>) -> usize {
    prog[1] = 12;
    prog[2] = 2;
    eval(prog)
}

pub fn day02_a() -> Result<String, String> {
    Ok(solve_a(parsed_lines("inputs/02.txt")?).to_string())
}

fn solve_b(prog: Vec<usize>) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut prog_ = prog.to_vec();
            prog_[1] = noun;
            prog_[2] = verb;
            if eval((&prog_).to_vec()) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0   
}

pub fn day02_b() -> Result<String, String> {
    Ok(solve_b(parsed_lines("inputs/02.txt")?).to_string())
}
