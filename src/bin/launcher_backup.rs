#![windows_subsystem = "windows"]
use std::{env, thread};
use std::process::{Command, Stdio};
use std::time::Duration;

fn main() {

    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap();
    let app_path = wd.join("Group13");
    let out = Command::new(app_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to run Group13");
/*
    #[cfg(target_os = "macos")]{
        Command::new("osascript")
                .arg("-e")
                .arg("tell application \"Terminal\" to set visible of front window to false")
                .output()
                .expect("Failed to hide terminal");
        //let mut kill_command = Command::new("kill").arg("-9").arg(std::process::id().to_string()).output().expect("Terminal not closed");
    }*/

    std::process::exit(0);

}
