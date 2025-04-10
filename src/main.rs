mod help;
mod instructionCounter;
mod runner;

use help::banner::help_banner;
use crate::runner::exe_runner::{run_exe_cmdargs, run_exe_user_input};

fn main(){

//     HELP banner
    let cmd_args: Vec<String> = std::env::args().skip(1).collect();
    if (cmd_args.len() < 3){
        help_banner();
        std::process::exit(1);
    }

//     Getting the name of the executable
    let exe_name = &cmd_args[0];
    let mode = &cmd_args[1];
    let mut input_vector = &cmd_args[2..].to_vec(); //make a vector from the 3rd index to the last of the inputs given.

    match mode.as_str() {
        "-cmdarg" => run_exe_cmdargs(exe_name.clone().to_string(),input_vector),
        "-userinput" => run_exe_user_input(exe_name.clone().to_string(),input_vector),
        _ => help_banner()
    }
}