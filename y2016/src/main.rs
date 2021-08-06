use std::env;
mod d1;
mod d2;
mod d3;

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
        "1" => d1::one(args),
        "2" => d2::two(args),
        "3" => d3::three(args),
        _ => not_implemented(day),
    };
    println!("Outputs: {:?}", out);
}

fn not_implemented(day: String) -> (String, String) {
    println!("⚠️  Function '{}' not implemented", day);
    ("ERROR".to_string(), "ERROR".to_string())
}
