use tracing_subscriber::fmt::layer;
use crate::run_popup;

#[derive(Debug)]
pub enum TrackingResult {
    InPrevRect,         // mouse is now in the previous rect / continue
    InCurrentRect,      // mouse is now in the current rect  / continue
    InNextRect,         // mouse is now in the next rect     / continue
    FinishedRectShape,  // mouse is now in the last rect!    / go to next shape
    FinishedMinusShape,  // mouse is now in the last rect!    / backup!
    OutOfTrack,         // mouse is now out of given track   / reset all
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Copy, Clone, Debug)]
pub struct Rectangular {
    pub top_sx: Point,
    pub bot_rx: Point,
}

impl Rectangular {
    pub fn new(tsx: Point, bry: Point) -> Self {
        Rectangular{ top_sx: tsx, bot_rx: bry }
    }

    pub fn is_in(&self, x: i32, y: i32) -> bool {
        let cond_x: bool = x > self.top_sx.x && x < self.bot_rx.x;
        let cond_y: bool = y < self.top_sx.y && y > self.bot_rx.y;
        cond_x && cond_y
    }
}

#[derive(Debug)]
pub struct MouseTracker {
    id: u8, // 0: RECT, 1: MINUS
    width: i32,
    height: i32,
    track: Vec<Rectangular>,
    current_index: usize,  // index in track vector
}

impl MouseTracker {
    pub fn new(w: i32, h: i32) -> Self {
        MouseTracker {
            id: 0,
            width: w,
            height: h,
            track: init_rect(w, h),
            current_index: 0,
        }
    }

    pub fn re_init(&mut self) -> () {
        self.id = 0;
        self.current_index = 0;
        self.track = init_rect(self.width, self.height);
    }

    pub fn track_point(&mut self, p: Point) -> TrackingResult {
        let curr_rect: Rectangular = self.track[self.current_index].clone();

        // mouse still in current rect
        if point_in_rect(&p, &curr_rect) {
            return TrackingResult::InCurrentRect;

            // mouse in the previous rect
        } else if self.current_index > 0 && point_in_rect(&p, &self.track[self.current_index - 1]) {
            return TrackingResult::InPrevRect;

            // mouse in next rect -> update current_index and check for last
        } else if self.current_index + 1 < self.track.len() && point_in_rect(&p, &self.track[self.current_index + 1]) {
            self.current_index += 1;
            // in last rect
            if self.current_index != self.track.len() - 1 {
                return TrackingResult::InNextRect;
                // not the last rect
            } else {

                if self.id == 0 { // RECT
                    self.id = 1;
                    self.current_index = 0;
                    self.track = init_minus(self.width, self.height);
                    return TrackingResult::FinishedRectShape;
                } else {          // MINUS
                    self.id = 0;
                    self.current_index = 0;
                    self.track = init_rect(self.width, self.height);
                    return TrackingResult::FinishedMinusShape;
                }
            }
        }

        // re-init and return out of track
        if self.id == 0 {
            self.id = 0;
            self.current_index = 0;
            self.track = init_rect(self.width, self.height);
        }

        TrackingResult::OutOfTrack
    }
}

pub fn point_in_rect(p: &Point, r: &Rectangular) -> bool {
    return p.x >= r.top_sx.x &&
        p.x <= r.bot_rx.x &&
        p.y >= r.top_sx.y &&
        p.y <= r.bot_rx.y;
}

fn init_rect(w: i32, h: i32) -> Vec<Rectangular> {
    let mut track: Vec<Rectangular> = Vec::<Rectangular>::new();

    let min_cells = 5;
    let size = (i32::min(h, w) + min_cells - 1) / min_cells;
    let limit = size * 2 / 3;


    // Oxy in top left corner
    {
        // vertical, left line
        let mut i = 0;
        while i < h {
            if h < i + size || h - (i + size) < limit {
                track.push(Rectangular {
                    top_sx: Point { x: -10, y: i },
                    bot_rx: Point { x: size-1, y: h + 10 },
                });
                i = h;
            } else {
                track.push(Rectangular {
                    top_sx: Point { x: -10, y: i },
                    bot_rx: Point {
                        x: size-1,
                        y: i32::min(h +10, i + size-1),
                    },
                });
                i += size;
            }
        }

        // horizontal, bottom line
        i = size;
        while i < w {
            if w < i + size || w - (i + size) < limit {
                track.push(Rectangular {
                    top_sx: Point { x: i, y: h - size },
                    bot_rx: Point { x: w +10, y: h +10 },
                });
                i = w;
            } else {
                track.push(Rectangular {
                    top_sx: Point { x: i, y: h - size },
                    bot_rx: Point {
                        x: i32::min(w +10, i + size-1),
                        y: h +10
                    },
                });
                i += size;
            }
        }

        // vertical, right line
        let mut i = h - size -1;
        while i > 0 {
            if i < size || i - size < limit {
                track.push(Rectangular {
                    top_sx: Point { x: w - size, y: -10 },
                    bot_rx: Point { x: w +10, y: i },
                });
                i = 0;
            } else {
                track.push(Rectangular {
                    top_sx: Point {
                        x: w - size,
                        y: if i >= size-1 { i - size +1 } else { -10 } },
                    bot_rx: Point { x: w +10, y: i },
                });
                i -= size;
            }
        }

        // horizontal, top line
        i = w - size -1;
        while i > 0 {
            if i < size || i - size < limit {
                track.push(Rectangular {
                    top_sx: Point { x: -10, y: -10 },
                    bot_rx: Point { x: i, y: size -1 },
                });
                i = 0;
            } else {
                track.push(Rectangular {
                    top_sx: Point {
                        x: if i >= size - 1 { i - size + 1 } else { -10 },
                        y: -10 },
                    bot_rx: Point { x: i, y: size -1 },
                });
                i -= size;
            }
        }
    }

    track
}

fn init_minus(w: i32, h: i32) -> Vec<Rectangular> {
    let mut track: Vec<Rectangular> = Vec::<Rectangular>::new();

    let min_cells = 10;
    let size = (i32::min(h, w) + min_cells - 1) / min_cells;
    let limit = size * 2 / 3;

    // first rect
    track.push(Rectangular {
        top_sx: Point { x: -10, y: -10 },
        bot_rx: Point { x: size -1, y: h +10 },
    });

    // horizontal, bottom line
    let mut i = size;
    while i < w {
        if w < i + size || w - (i + size) < limit {
            track.push(Rectangular {
                top_sx: Point { x: i, y: -10 },
                bot_rx: Point { x: w +10, y: h +10 },
            });
            i = w;
        } else {
            track.push(Rectangular {
                top_sx: Point { x: i, y: -10 },
                bot_rx: Point {
                    x: i32::min(w +10, i + size-1),
                    y: h +10
                },
            });
            i += size;
        }
    }

    track
}


