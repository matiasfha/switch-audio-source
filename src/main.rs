use std::process::{Command, Stdio};
use execute::Execute;

const SWITCH_AUDIO_SOURCE_PATH: &str = "/opt/homebrew/bin/SwitchAudioSource";

fn main() {

    let mut current_command = Command::new(SWITCH_AUDIO_SOURCE_PATH);
    current_command.arg("-c"); 
    current_command.stdout(Stdio::piped());
current_command.stderr(Stdio::piped());
    let current = current_command.execute_output().unwrap();
    let current_str =String::from_utf8(current.stdout).unwrap();
    println!("{}", current_str);
    
    let mut command = Command::new(SWITCH_AUDIO_SOURCE_PATH);

    if current_str == String::from("Mi Monitor\n") {
        command.args(["-s" ,"Scarlett Solo USB"]);
    }else{
        command.args(["-s", "Mi Monitor"]);
    }
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

let output = command.execute_output().unwrap();
    if let Some(exit_code) = output.status.code() {
    if exit_code == 0 {
        println!("Ok.");
    } else {
        eprintln!("Failed.");
    }
} else {
    eprintln!("Interrupted!");
}
    println!("{}", String::from_utf8(output.stdout).unwrap());
println!("{}", String::from_utf8(output.stderr).unwrap());



        
}
