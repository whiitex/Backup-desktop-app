use std::process::Command;
use Group13::manage_movement;
fn main() {

    let mut gui=Command::new("spawn_gui")
        .spawn()
        .expect("Failed to execute process");


    manage_movement();

    gui.wait().expect("Failed to wait on child process");
    println!("Main executed successfully!")
}