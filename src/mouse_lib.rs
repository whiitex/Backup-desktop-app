use std::process::Command;
use rdev::{listen, Event, EventType};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::RecvError;
use crate::mouse_tracker::{MouseTracker, Point};
use crate::{Choice, TrackingResult};
use std::thread;
use crate::fs_copy::do_backup;
use crate::run_popup;

pub enum Shape {
    Rect,
    Minus
}
pub fn manage_movement() {
    let size = match rdev::display_size() {
        Ok(s) => {s},
        Err(e) => {
            println!("Error retrieving display size: {:?}", e);
            return;
        }
    };

    // Crea un `MouseTracker` inizializzato con la dimensione dello schermo
    let tracker = Arc::new(Mutex::new((MouseTracker::new(size.0 as i32, size.1 as i32), 0, 0, Shape::Rect)));

    // Clona l'`Arc` per poterlo usare all'interno della closure
    let tracker_clone = Arc::clone(&tracker);

    // Funzione di callback per gestire gli eventi
    let callback = move |event: Event| {
        match event.event_type {
            EventType::MouseMove { x, y } => {
                let point = Point {
                    x: x.trunc() as i32,
                    y: y.trunc() as i32,
                };

                // Usa `tracker_clone` all'interno della closure
                let mut tracker = tracker_clone.lock().unwrap();
                if tracker.1 != point.x || tracker.2 != point.y {
                    // match sulla shape per chiamare track_point o track_minus
                    let res = tracker.0.track_point(point);
                    tracker.1 = point.x;
                    tracker.2 = point.y;
                    //println!("{:?}, {:?}", res, point);
                    //println!("{:?}", tracker.0);
                    match res {
                        TrackingResult::FinishedRectShape => {

                            tracker.3 = Shape::Minus;


                            let child =Command::new("cargo")
                                .arg("run")
                                .arg("--bin")
                                .arg("spawn_popup")
                                .spawn()
                                .expect("Failed to execute process");

                            drop(tracker);

                            let tracker_clone1 = Arc::clone(&tracker_clone);

                            thread::spawn(move ||{

                                match child.wait_with_output().unwrap().status.code().unwrap() {
                                    1 => {
                                        println!("Backup started");
                                        //do_backup();
                                        println!("Backup done");
                                    },
                                    _ => {
                                        println!("Backup not started");
                                    },
                                }
                            });
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    };

    // Avvia l'ascolto degli eventi
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}