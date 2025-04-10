// This file is responsible for running the executable with its desired inputs.
// Also it chooses whether the executable wants the input as command line arguments or simply as input from the user format.


use std::process::{Command,Output,Stdio};
use std::io;
use std::io::Write;

pub fn run_exe_user_input(exe_name: String, input: &Vec<String>){
    let mut final_input: &[u8] = &[];
    let default_input = b"0123456789";
    if input.is_empty(){
        //TODO: run the exe with default input
        //We will just get the input and put it inside stdin of the executable
        final_input = default_input;
    }
    else{
        final_input = input[0].as_bytes();
    }
    // println!("{:?}",final_input);
    let mut exe_process = Command::new(exe_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start the executable!!");

    if let Some(mut stdin) = exe_process.stdin.take(){
        stdin.write_all(final_input).unwrap();
    }

    let output = exe_process
        .wait_with_output()
        .expect("Failed to read the output!!");

    println!("Output: {}\n",String::from_utf8_lossy(&output.stdout));

}
pub fn run_exe_cmdargs(exe_name: String,input: &Vec<String>){
    let output = Command::new(exe_name)
        .args(input)
        .output();
    println!("{:?}", output);
    if output.unwrap().status.success(){
        println!("program ran successfully...");
    }
    else {
        eprintln!("Program exited with error!!");
    }
}