use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem {0}", err);
        process::exit(1);
    });

   if let Err(e) = minigrep::run(config){
    eprintln!("Found error while running: {0}", e);
    process::exit(1);
   }

}

