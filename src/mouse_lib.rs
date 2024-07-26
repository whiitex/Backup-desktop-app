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
    let tracker = Arc::new(Mutex::new((MouseTracker::new(size.0 as u32, size.1 as u32), 0, 0, Shape::Rect)));

    // Clona l'`Arc` per poterlo usare all'interno della closure
    let tracker_clone = Arc::clone(&tracker);
    let tracker_clone1 = Arc::clone(&tracker);

    // Funzione di callback per gestire gli eventi
    let callback = move |event: Event| {
        match event.event_type {
            EventType::MouseMove { x, y } => {
                let point = Point {
                    x: x.trunc() as u32,
                    y: y.trunc() as u32,
                };

                // Usa `tracker_clone` all'interno della closure
                let mut tracker = tracker_clone.lock().unwrap();
                if tracker.1 != point.x || tracker.2 != point.y {
                    // match sulla shape per chiamare track_point o track_minus
                    let res = tracker.0.track_point(point);
                    tracker.1 = point.x;
                    tracker.2 = point.y;
                    println!("{:?}, {:?}", res, point);
                    //println!("{:?}", tracker.0);
                    /*match res {
                        TrackingResult::FinishedRectShape => {

                            tracker.3 = Shape::Minus;

                            let (sender, receiver) = std::sync::mpsc::channel();

                            run_popup(sender);
                            drop(tracker);
                            thread::spawn(move ||{
                                let tracker = tracker_clone1.lock().unwrap();

                                match receiver.recv() {
                                    Ok(v) => {
                                        match v {
                                            Choice::Yes => {
                                                do_backup();
                                                tracker.3 = Shape::Rect;
                                            }
                                            Choice::No => {
                                                tracker.3 = Shape::Rect;
                                            }
                                        }
                                    }
                                    Err(_) => {tracker.3 = Shape::Rect;}
                                }
                            });
                        },
                        _ => {}
                    }*/
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