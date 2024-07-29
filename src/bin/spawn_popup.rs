#![windows_subsystem = "windows"]

use std::thread;
use Group13::{Choice, run_popup};
fn main() {
    let (sender, receiver) = std::sync::mpsc::channel();

    thread::spawn(move||{
        match receiver.recv() {
            Ok(choice) => {
                match choice {
                    Choice::Yes => {
                        print!("1");
                        std::process::exit(0);
                    }
                    Choice::No => {
                        print!("2");
                        std::process::exit(0);
                    }
                }
            }
            Err(_) => {
                print!("3");
                std::process::exit(0);
            }
        }
    });

    run_popup(sender,false);

}