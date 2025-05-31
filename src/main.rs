use app::cli::cli_handler;
use std::env;


mod app;

fn main() {
    //getting the argument and removing the 0th arg (program name)
    let mut args:Vec<String> = env::args().collect(); args.reverse(); args.pop();args.reverse();
    if args.len() == 0{
        args.push("app".to_string());
        cli_handler(&mut args);
    } else{
        cli_handler(&mut args);
    }

}


