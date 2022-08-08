use application::Application;
use sfml::{graphics::RenderWindow, window::Style};

mod application;
mod keyboard;

//mod scene;

fn main() {
    let mut app = Application::new(1280, 720, String::from("eng"), true);
    app.main();
}
