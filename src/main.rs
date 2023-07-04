use std::process::Command;
use std::fs::{self};
use inquire::{error::InquireError, Select};



fn main() {
    let dir = "/etc/dm-chooser/";
    let mut string_options = Vec::new();
    let mut options = Vec::<&str>::new();

    let read_result = fs::read_dir(dir).unwrap();
    
    for entry in read_result {
        string_options.push(entry.unwrap().path().display().to_string());
    }

    for i in 0..string_options.len() {
        options.push(string_options[i].as_str())
    }

    let ans: Result<&str, InquireError> = Select::new("Hello yosyo, welcome back ! On which Wayland Compositor would you like to go ? ", options).prompt();

    let mut worked = true;
    let mut choosed_wm = "";

    match ans {
         Ok(choice) => choosed_wm = choice,
         Err(_) => worked = false,
    };

    if worked {
        Command::new(&choosed_wm)
                .spawn()
                .expect("WM command failed to start");
    };
}
