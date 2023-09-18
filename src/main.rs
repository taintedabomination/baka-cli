use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let total_args = args.len();

    if total_args <= 1 {
        eprintln!("Použití: baka-cli <prikaz>");
        std::process::exit(1);
    }

    println!("Použitý příkaz: {}", args[1]);
}
