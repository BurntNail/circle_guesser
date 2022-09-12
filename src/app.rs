use piston_window::{clear, Context, DrawState, Ellipse, G2d, Graphics};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

const SCALE: f64 = 5.0;
const PADDING: f64 = 0.1;
const MIN_PCTG: f64 = 0.2;
const START_HINTS: usize = 3;

pub struct CircleGuesserApp {
    current_hints: Vec<[f64; 2]>,
    is_revealed: Option<[f64; 2]>, //holds the mouse pos
    needs_to_report_score: bool,
    current_circle: (f64, f64, f64), //x, y, radius
    old_size: [f64; 2],
    gen: ThreadRng,
}

impl CircleGuesserApp {
    pub fn new(win_size: [f64; 2]) -> Self {
        let mut s = Self {
            current_hints: vec![[0.0, 0.0]; START_HINTS],
            is_revealed: None,
            needs_to_report_score: false,
            current_circle: (0.0, 0.0, 0.0),
            old_size: [0.0, 0.0],
            gen: thread_rng(),
        };
        s.get_new_values(Some(win_size));
        s
    }

    fn get_point_on_circle(gen: &mut ThreadRng, circle: (f64, f64, f64)) -> [f64; 2] {
        let (x, y, radius) = circle;
        let top_right_from_d = |d| {
            let d = f64::from(d);
            let from_right_flat = 90.0 - d;
            [
                radius * from_right_flat.to_radians().cos(),
                radius * from_right_flat.to_radians().sin(),
            ]
        };

        let degrees = gen.gen_range(0..360_u32);
        let [rx, ry] = match degrees {
            0 => [0.0, -radius],
            1..=89 => {
                //top right
                top_right_from_d(degrees)
            }
            90 => [radius, 0.0],
            91..=179 => {
                //btm right
                let coords = top_right_from_d(degrees - 90);
                [coords[0], -coords[1]]
            }
            180 => [0.0, radius],
            181..=269 => {
                //btm left
                let coords = top_right_from_d(degrees - 180);
                [-coords[0], -coords[1]]
            }
            270 => [-radius, 0.0],
            271..=359 => {
                //top left
                let coords = top_right_from_d(degrees - 270);
                [-coords[0], coords[1]]
            }
            _ => unreachable!("degrees messed up: {degrees}"),
        };
        [x + rx, y + ry]
    }

    pub fn get_new_values(&mut self, win_size: Option<[f64; 2]>) {
        if let Some(ws) = win_size {
            self.old_size = ws;
        }
        self.is_revealed = None;

        let x_size = self.old_size[0] / SCALE;
        let y_size = self.old_size[1] / SCALE;

        let chosen_x = f64::from(self.gen.gen_range({
            let min_x = (x_size * PADDING) as u32;
            let max_x = (x_size as u32) - min_x;
            min_x..max_x
        }));
        let chosen_y = f64::from(self.gen.gen_range({
            let min_y = (y_size * PADDING) as u32;
            let max_y = (y_size as u32) - min_y;
            min_y..max_y
        }));

        let min_dist_to_edge = (x_size - chosen_x)
            .min(y_size - chosen_y)
            .min(chosen_x)
            .min(chosen_y);
        
        
        let radius = f64::from({
            let start = (min_dist_to_edge * MIN_PCTG) as u32;
            let end = min_dist_to_edge
            self
                .gen
                .gen_range(start..end)
        });

        self.current_circle = (chosen_x, chosen_y, radius);

        self.current_hints = (0..self.current_hints.len())
            .map(|_| Self::get_point_on_circle(&mut self.gen, self.current_circle))
            .collect();
        self.needs_to_report_score = true;
    }

    pub fn more_pts(&mut self) {
        self.current_hints.push(Self::get_point_on_circle(
            &mut self.gen,
            self.current_circle,
        ));
    }
    pub fn less_pts(&mut self) {
        if self.current_hints.len() > 1 {
            self.current_hints.remove(0);
        }
    }

    pub fn render(&mut self, ctx: Context, graphics: &mut G2d, window_size: [f64; 2]) {
        clear([0.0; 4], graphics);

        if self.old_size != window_size || self.old_size == [0.0; 2] {
            self.get_new_values(Some(window_size));
        }

        let t = ctx.transform;
        for pos in &self.current_hints {
            let ellipse = Ellipse::new([0.0, 1.0, 0.0, 1.0]);
            let rect = [pos[0] * SCALE, pos[1] * SCALE, SCALE, SCALE];
            graphics.ellipse(&ellipse, rect, &DrawState::default(), t);
        }

        if let Some([mx, my]) = self.is_revealed {
            let (cx, cy, rad) = self.current_circle;
            let size = SCALE.min(2.0);
            {
                //circle
                let circle = Ellipse::new_border([1.0, 0.0, 0.0, 1.0], SCALE);
                let rect = [cx * SCALE, cy * SCALE, SCALE, SCALE];
                graphics.ellipse(&circle, rect, &DrawState::default(), t);

                let ellipse = Ellipse::new_border([0.75, 0.75, 0.75, 0.5], size);
                let rect = [
                    (cx - rad) * SCALE,
                    (cy - rad) * SCALE,
                    rad * 2.0 * SCALE,
                    rad * 2.0 * SCALE,
                ];
                graphics.ellipse(&ellipse, rect, &DrawState::default(), t);
            }
            {
                //mouse
                let ellipse = Ellipse::new([0.0, 0.0, 1.0, 1.0]);
                let rect = [mx, my, SCALE * 2.0, SCALE * 2.0];
                graphics.ellipse(&ellipse, rect, &DrawState::default(), t);
            }
            if self.needs_to_report_score {
                let distance = (mx - cx * SCALE).hypot(my - cy * SCALE);
                if distance == 0.0 {
                    println!("Perfect Score!");
                } else {
                    let win_av_size = window_size[0].hypot(window_size[1]);
                    println!(
                        "You were {distance:.1} away - that is only {:.1}% of the screen size!",
                        distance / win_av_size * 100.0
                    )
                }
                self.needs_to_report_score = false;
            }
        }
    }

    pub fn mouse_input(&mut self, pos: [f64; 2]) {
        if self.is_revealed.is_none() {
            self.is_revealed = Some(pos);
        } else {
            println!("Need to Clear using C");
        }
    }
}
