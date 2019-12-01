mod days;
mod util;

fn main() -> Result<(), String> {
    let days = days::all_days();

    let args = util::get_arguments()?;
    if args.part.is_some() {
        let solution = days[args.day - 1][args.part.unwrap()]()?;
        println!("Solution {}: {}", args, solution);
    } else {
        let solution_a = days[args.day - 1][0]()?;
        println!("Solution {}a: {}", args.day, solution_a);
        let solution_b = days[args.day - 1][1]()?;
        println!("Solution {}b: {}", args.day, solution_b);
    }

    Ok(())
}
