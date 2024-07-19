use std::process::Command;

fn main() {

    let mut gui=Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("GUI")
        .spawn()
        .expect("Failed to execute process");


    gui.wait().expect("Failed to wait on child process");
    println!("Main executed successfully!")
}