mod mouse_tracker {

    pub enum TrackingResult {
        InPrevRect,         // mouse is now in the previous rect / continue
        InCurrentRect,      // mouse is now in the current rect  / continue
        InNextRect,         // mouse is now in the next rect     / continue
        FinishedRectShape,  // mouse is now in the last rect!    / go to next shape
        OutOfTrack,         // mouse is now out of given track   / reset all
    }

    #[derive(Copy, Clone)]
    struct Point {
        x: u32,
        y: u32,
    }
    #[derive(Copy, Clone)]
    struct Rectangular {
        top_sx: Point,
        bot_rx: Point,
    }
    
    impl Rectangular {
        pub fn new(tsx: Point, bry: Point) -> Self {
            Rectangular{ top_sx: tsx, bot_rx: bry }
        }
        
        pub fn is_in(&self, x: u32, y: u32) -> bool {
            let cond_x: bool = x > self.top_sx.x && x < self.bot_rx.x;
            let cond_y: bool = y < self.top_sx.y && y > self.bot_rx.y;
            cond_x && cond_y
        }
    }
    
    pub struct MouseTracker {
        width: u32,
        height: u32,
        track: Vec<Rectangular>,
        current_index: usize,  // index in track vector
    }
    

    pub fn point_in_rect(p: &Point, r: &Rectangular) -> bool {
        return p.x >= r.top_sx.x &&
            p.x <= r.bot_rx.x &&
            p.y <= r.top_sx.y &&
            p.y >= r.bot_rx.y;
    }


    /*
    For this tracker we are considering Oxy is located in bottom left of the screen
     */
    impl MouseTracker {
        pub fn new(w: u32, h: u32) -> Self {
            let mut track: Vec<Rectangular> = Vec::<Rectangular>::new();

            let size = u32::min(h, w) / 25;

            // vertical, left line
            let mut i = h;
            while i > 0 {
                if i - size < size * 2 / 3 {
                    track.push(Rectangular {
                        top_sx: Point { x: 0, y: i },
                        bot_rx: Point { x: size, y: 0 },
                    });
                    i = 0;
                } else {
                    track.push(Rectangular {
                        top_sx: Point { x: 0, y: i },
                        bot_rx: Point { x: size, y: i-size },
                    });
                    i -= size;
                }
            }

            // horizontal, bottom line
            i = size;
            while i < w {
                if w - (i + size) < size * 2 / 3 {
                    track.push(Rectangular {
                        top_sx: Point { x: i, y: size },
                        bot_rx: Point { x: w, y: 0 },
                    });
                    i = w;
                } else {
                    track.push(Rectangular {
                        top_sx: Point { x: i, y: size },
                        bot_rx: Point { x: i + size, y: 0 },
                    });
                    i += size;
                }
            }

            // vertical, right line
            i = size;
            while i < h {
                if h - (i + size) < size * 2 / 3 {
                    track.push(Rectangular {
                        top_sx: Point { x: w - size, y: h },
                        bot_rx: Point { x: w, y: i },
                    })
                } else {
                    track.push(Rectangular {
                        top_sx: Point { x: w - size, y: i + size },
                        bot_rx: Point { x: w, y: i },
                    })
                }
            }

            // horizontal, top line
            i = w-size;
            while i > 0 {
                if i - size < size * 2 / 3 {
                    track.push(Rectangular {
                        top_sx: Point { x: 0, y: h },
                        bot_rx: Point { x: i, y: h - size },
                    });
                    i = 0;
                } else {
                    track.push(Rectangular {
                        top_sx: Point { x: i - size, y: h },
                        bot_rx: Point { x: i, y: h - size },
                    });
                    i -= size;
                }
            }

            MouseTracker {
                width: w,
                height: h,
                track: track,
                current_index: 0,
            }
        }

        pub fn is_in(&mut self, p: Point) -> TrackingResult {
            let curr_rect: Rectangular = self.track[self.current_index].clone();

            // mouse still in current rect
            if point_in_rect(&p, &curr_rect) {
                return TrackingResult::InCurrentRect;

            // mouse in the previous rect
            } else  if self.current_index > 0 && point_in_rect(&p, &self.track[self.current_index - 1]) {
                return TrackingResult::InPrevRect;

            // mouse in next rect -> update current_index and check for last
            } else if self.current_index + 1 < self.track.len() && point_in_rect(&p, &self.track[self.current_index + 1]) {
                self.current_index += 1;

                // in last rect
                if self.current_index + 1 == self.track.len() {
                    return TrackingResult::FinishedRectShape;
                // not the last rect
                } else { return TrackingResult::InNextRect; }
            }

            return TrackingResult::OutOfTrack;
        }
    }
}