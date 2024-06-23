extern crate piston;
extern crate piston_window;
extern crate find_folder;

use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::{RenderEvent, UpdateEvent};
use piston_window::{PistonWindow, WindowSettings, clear};
use find_folder::Search;

pub struct App {
    // Add fields if needed for the state of the app
}

impl App {
    fn render(&mut self, window: &mut PistonWindow, event: &piston_window::Event) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        window.draw_2d(event, |c, g, _| {
            clear(GREEN, g);
        });
    }

    fn update(&mut self) {
        // Update the app's state here
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        // Initialize fields if needed
    };

    let mut events = Events::new(EventSettings::new()).ups(60);

    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            app.render(&mut window, &e);
        }

        if let Some(_) = e.update_args() {
            app.update();
        }
    }
}
