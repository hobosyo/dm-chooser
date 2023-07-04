use std::process::Command;

use inquire::{error::InquireError, Select};



fn main() {
    let raw_output = Command::new("/bin/ls")
                            .arg("-1")
                            .arg("-A")
                            .arg("/etc/lemurs/wayland/")
                            .output()
                            .expect("ls command failed to start");

    let _output = String::from_utf8(raw_output.stdout).unwrap();

    let options: Vec<&str> = vec!["Hyprland", "dwl",];


    let ans: Result<&str, InquireError> = Select::new("Hello yosyo, welcome back ! On which Wayland Compositor would you like to go ? ", options).prompt();

    let mut worked = true;
    let mut choosed_wm = "";

    match ans {
         Ok(choice) => choosed_wm = choice,
         Err(_) => worked = false,
    };

    if worked {
        println!("{choosed_wm}");
        Command::new(choosed_wm)
                .spawn()
                .expect("WM command failed to start");
    };
}
