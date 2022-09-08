use piston_window::{Button, Key, MouseButton, MouseCursorEvent, PistonWindow, PressEvent, RenderEvent, WindowSettings};
use crate::app::CircleGuesserApp;

mod app;

fn main() {
    let start_size = [800.0, 800.0];
    let mut app = CircleGuesserApp::new(start_size);
    let mut win: PistonWindow = WindowSettings::new("Circle Guesser", start_size)
        .exit_on_esc(true)
        .resizable(true)
        .build().expect("building window");
    
    let mut mouse_pos = [0.0, 0.0];
    while let Some(e) = win.next() {
        if let Some(r) = e.render_args() {
            win.draw_2d(&e, |c, g, _device| {
                app.render(c, g, r.window_size);
            });
        }
        
        if let Some(pa) = e.press_args() {
            match pa {
                Button::Keyboard(kb) => if kb == Key::C {
                    app.get_new_values(None);
                },
                Button::Mouse(m) => match m {
                    MouseButton::Left => {
                        app.mouse_input(mouse_pos);
                    },
                    MouseButton::Right => {
                        app.get_new_values(None);
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        
        e.mouse_cursor(|p| mouse_pos = p);
    }
}
