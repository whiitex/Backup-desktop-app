use std::process::Command;
use Group13::manage_events;
fn main() {

    let mut gui=Command::new("A")
        .spawn()
        .expect("Failed to execute process");



    manage_events();

    gui.wait().expect("Failed to wait on child process");
    println!("Main executed successfully!")
}