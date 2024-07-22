use std::process::Command;

fn main() {

    let mut gui=Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("gui_example")
        .spawn()
        .expect("Failed to execute process");

    let mut popup=Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("popup_example")
        .spawn()
        .expect("Failed to execute process");


    popup.wait().expect("Failed to wait on child process");
    gui.wait().expect("Failed to wait on child process");
    println!("Main executed successfully!")
}