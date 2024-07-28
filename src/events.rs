use std::path::Path;
use std::process::Command;
use rdev::{listen, Event, EventType, Key};
use std::sync::{Arc, Mutex};
use crate::mouse_tracker::{MouseTracker, Point};
use crate::{TrackingResult};
use std::{env, thread};
use crate::fs_copy::do_backup;

pub fn manage_events() {
    let size = match rdev::display_size() {
        Ok(s) => {s},
        Err(e) => {
            println!("Error retrieving display size: {:?}", e);
            return;
        }
    };

    // Stato utilizzato per gestire il tasto shift
    let state = Arc::new(Mutex::new(0));

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
                    let pid = tracker.3;
                    match res {
                        TrackingResult::FinishedRectShape => {


                            // let out_dir = std::env::var("OUT_DIR").unwrap();
                            let exe = env::current_exe().unwrap();
                            let len = exe.clone().to_str().unwrap().len();
                            let wd: String = exe.to_str().unwrap().chars().take(len-7).collect::<Vec<char>>().into_iter().collect();
                            let popup_path = Path::new(&wd).join("spawn_popup");

                            let child = Command::new(popup_path.to_str().unwrap())
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
            EventType::KeyPress(key) => {
                let mut state = state.lock().unwrap();
                if key == Key::ShiftLeft || key == Key::ShiftRight {
                    *state = 1;
                }
                else if key == Key::KeyA && *state == 1 {
                    println!("Shift+A pressed");
                    Command::new("spawn_gui")
                        .spawn()
                        .expect("Failed to execute process");
                    *state = 0;
                }
                else {
                    *state = 0;
                }
            },
            EventType::KeyRelease(key) => {
                match key {
                    Key::ShiftLeft | Key::ShiftRight => {
                        let mut state = state.lock().unwrap();
                        *state = 0;
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    };

    // Start event listening
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}