use self::Mode::*;
use self::Op::*;

pub enum Mode {
    Position,
    Immediate,
}

pub struct Arg(i32, Mode);

impl Arg {
    pub fn position(value: i32) -> Arg {
        Arg(value, Position)
    }

    pub fn immediate(value: i32) -> Arg {
        Arg(value, Immediate)
    }
}

pub enum Op {
    Halt,
    Add(Arg, Arg, Arg),
    Mul(Arg, Arg, Arg),
    Input(Arg),
    Output(Arg),
}

impl Op {
    pub fn halt() -> Op {
        Halt
    }

    pub fn add(r1: i32, r2: i32, out: i32) -> Op {
        Op::add_ex(Arg::position(r1), Arg::position(r2), Arg::position(out))
    }

    pub fn add_ex(r1: Arg, r2: Arg, out: Arg) -> Op {
        Add(r1, r2, out)
    }

    pub fn mul(r1: i32, r2: i32, out: i32) -> Op {
        Op::mul_ex(Arg::position(r1), Arg::position(r2), Arg::position(out))
    }

    pub fn mul_ex(r1: Arg, r2: Arg, out: Arg) -> Op {
        Mul(r1, r2, out)
    }

    pub fn input(out: i32) -> Op {
        Op::input_ex(Arg::position(out))
    }

    pub fn input_ex(out: Arg) -> Op {
        Input(out)
    }

    pub fn output(out: i32) -> Op {
        Op::output_ex(Arg::position(out))
    }

    pub fn output_ex(r: Arg) -> Op {
        Output(r)
    }

    pub fn size(&self) -> usize {
        match self {
            Halt => 1,
            Add(_, _, _) => 4,
            Mul(_, _, _) => 4,
            Input(_) => 2,
            Output(_) => 2,
        }
    }

    pub fn parse(input: &str) -> Result<Vec<Op>, String> {
        Err("Not implemented".to_string())
    }
}

pub struct Machine<'a> {
    pub memory: &'a [i32],
    pub instr: usize,
}

impl<'a> Machine<'a> {
    pub fn evaluate(
        &mut self,
        program: &'a [Op],
        inputs: &mut dyn std::iter::Iterator<Item = i32>,
    ) -> Result<&'a [i32], String> {
        Err("Not implemented".to_string())
    }
}

pub fn day05_a() -> Result<String, String> {
    Err("Unsolved".to_string())
}

pub fn day05_b() -> Result<String, String> {
    Err("Unsolved".to_string())
}
