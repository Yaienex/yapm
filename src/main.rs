use app::cli::cli_handler;
use std::env;
use std::process::exit;

mod app;

fn main() {
    //getting the argument and removing the 0th arg (program name)
    let mut args:Vec<String> = env::args().collect(); args.reverse(); args.pop();args.reverse();
    if args.len() == 0{
        println!("No arguments were given, type yapm help to get the list of the possible command");
        exit(1);
    }
    cli_handler(&mut args);

}


