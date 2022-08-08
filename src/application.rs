use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{Event, Key, Style};

pub struct Application {
    pub x: u16,
    pub y: u16,
    pub title: String,
    window: RenderWindow,
    pub vsync: bool,
}

impl Application {
    pub fn new(x: u16, y: u16, title: String, vsync: bool) -> Self {
        let mut win = RenderWindow::new(
            (x as u32, y as u32),
            title.as_str(),
            Style::CLOSE,
            &Default::default(),
        );
        win.set_vertical_sync_enabled(vsync);

        Self {
            x: x,
            y: y,
            title: title,
            window: win,
            vsync: vsync,
        }
    }

    pub fn main(&mut self) {
        loop {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::ESCAPE, ..
                    } => return,
                    _ => {}
                }
            }

            self.window.clear(Color::BLACK);
            self.window.display();
        }
    }
}
