use std::thread;
use Group13::{Choice, run_popup};
fn main() {
    let (sender, receiver) = std::sync::mpsc::channel();

    thread::spawn(move||{
        match receiver.recv() {
            Ok(choice) => {
                match choice {
                    Choice::Yes => {println!("User chose to proceed.")}
                    Choice::No => {println!("User chose not to proceed.")}
                }
            }
            Err(_) => {println!("Error: not proceeding.")}
        }
    });

    run_popup(sender);

}