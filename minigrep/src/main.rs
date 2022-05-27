use std::env;
use std::process::exit;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("failed parsing arguments: {}", err);
        exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

     if let Err(e) = minigrep::run(config) {
         eprintln!("Application error: {}", e);
         exit(1);
     }
}
