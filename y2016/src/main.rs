use std::env;
mod one;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("⚠️  Not enough input arguments");
        return;
    }
    args.remove(0); // remove target arg

    let day = args[0].clone(); // clone day from array
    args.remove(0); // remove day argument

    let out = match &day[..] {
        "1" => one::one(args),
        _ => not_implemented(day),
    };
    println!("Outputs: {:?}", out);
}

fn not_implemented(day: String) -> (isize, isize) {
    println!("⚠️  Function '{}' not implemented", day);
    (0, 0)
}
