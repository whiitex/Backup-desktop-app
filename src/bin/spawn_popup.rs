use std::thread;
use Group13::{Choice, run_popup};
fn main() {
    let (sender, receiver) = std::sync::mpsc::channel();

    thread::spawn(move||{
        match receiver.recv() {
            Ok(choice) => {
                match choice {
                    Choice::Yes => {
                        println!("User chose to proceed.");
                        std::process::exit(1);
                    }
                    Choice::No => {
                        println!("User chose not to proceed.");
                        std::process::exit(2);
                    }
                }
            }
            Err(_) => {
                println!("Error: not proceeding.");
                std::process::exit(3);
            }
        }
    });

    run_popup(sender);

}