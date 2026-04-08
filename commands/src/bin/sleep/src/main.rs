use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: sleep <seconds>");
        std::process::exit(1);
    }

    if let Ok(secs) = args[1].parse::<u64>() {
        thread::sleep(Duration::from_secs(secs));
    } else {
        eprintln!("invalid duration: {}", args[1]);
        std::process::exit(1);
    }
}
