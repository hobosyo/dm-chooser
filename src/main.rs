use std::process::Command;
use anyhow::Context;
use std::fs::{self};
use inquire::Select;

const DIRECTORY: &str = "/etc/dm-chooser";

fn main() -> anyhow::Result<()> {
    loop {
        let options = fs::read_dir(DIRECTORY)?
                        .map(|res| res.map(|e| e.path().display().to_string()))
                        .collect::<Result<Vec<_>, _>>()?;
    
        let choice = Select::new(
            "Hello yosyo, welcome back ! On which Wayland Compositor would you like to go ? ",
            options,
            )
            .prompt()?;

        let mut wm = Command::new(choice)
                .spawn()
                .context("WM command failed to start")?;

        let _result = wm.wait().unwrap();

        return Ok(());
    }
}
