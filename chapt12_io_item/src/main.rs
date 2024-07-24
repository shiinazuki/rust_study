use std::{env, process};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // run(&args).unwrap_or_else(|err| {
    //     println!("Problen parsing arguments: {}", err);
    //     process::exit(1);
    // });

    if let Err(e) = chapt12_io_item::run(env::args()) {
        eprintln!("Problen parsing arguments: {}", e);
        process::exit(1);
    }
}
