use rdev::{listen, Event, EventType};
use mouse_tracker::mouse_tracker::{MouseTracker, Point};

pub fn manage_movement(){

    let tracker : MouseTracker;

    // Funzione di callback per gestire gli eventi
    let callback = |event: Event| {
        match event.event_type {
            EventType::MouseMove { x, y } => {
                println!("Mouse moved to: ({}, {})", x, y);
                setPoint(Point{x: x.trunc() as u32, y: y.trunc() as u32});
            },
            _ => {}

        }
    };

    // Avvia l'ascolto degli eventi
    if let Err(error) = listen(callback) {
        let size = rdev::display_size();

        match &size {
            Ok(s) => {
                tracker = MouseTracker::new(s.0 as u32, s.1 as u32)
            }
            s => { println!("{:?}", s) }
        }

        println!("Error: {:?}", error);
    }

}

fn setPoint(p: Point) {
    //tracker.track_point(p);
}
