use std::io::{BufRead, Read};
use std::process::Command;
use rdev::{listen, Event, EventType};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::RecvError;
use crate::mouse_tracker::{MouseTracker, Point};
use crate::{Choice, TrackingResult};
use std::thread;
use std::io::BufReader;
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
    let tracker = Arc::new(Mutex::new((MouseTracker::new(size.0 as i32, size.1 as i32), 0, 0, None)));

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
                    println!("{:?}, {:?}", res, point);
                    //println!("{:?}", tracker.0);
                    let mut pid = tracker.3;
                    match res {
                        TrackingResult::FinishedRectShape => {



                            let child =Command::new("spawn_popup")
                                .stdout(std::process::Stdio::piped())
                                .spawn()
                                .expect("Failed to execute process");

                            /*let output = child.stdout.expect("Failed to read stdout");
                            let mut pipe_in= BufReader::new(output);
                            let mut buf = String::new();
                            pipe_in.read_line(&mut buf).unwrap();*/

                            tracker.3=Some(child.id());

                            println!("\x07");

                            drop(tracker);

                            let tracker_clone1 = Arc::clone(&tracker_clone);
                            thread::spawn(move ||{
                                let output= child.wait_with_output().unwrap();
                                let stdout = output.stdout;
                                let mut tracker = tracker_clone1.lock().unwrap();
                                tracker.0.re_init();
                                let conversion = String::from_utf8_lossy(&stdout).parse();
                                if let Ok(code) = conversion{
                                    println!("{}", code);
                                    match code {
                                        1 => {
                                            println!("Backup started");
                                            match do_backup(){
                                                Ok(_) => {
                                                    println!("Backup done");
                                                },
                                                Err(e) => {
                                                    println!("Backup failed: {:?}", e);
                                                }
                                            }
                                        },
                                        _ => {
                                            println!("Backup not started");
                                        },
                                    }
                                }

                            });
                        },
                        TrackingResult::FinishedMinusShape => {
                            println!("\x07");

                            println!("Backup started");
                            match do_backup() {
                                Ok(_) => {
                                    println!("Backup done");
                                },
                                Err(e) => {
                                    println!("Backup failed: {:?}", e);
                                }
                            }

                            #[cfg(target_os = "windows")]
                            {
                                if let Some(pid) = pid {
                                    Command::new("taskkill")
                                        .args(&["/PID", &pid.to_string(), "/F"])
                                        .output().unwrap();
                                }

                            }

                            #[cfg(not(target_os = "windows"))]
                            {
                                Command::new("kill")
                                    .args(&["-9", &pid.unwrap().to_string()])
                                    .output().unwrap();
                            }
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