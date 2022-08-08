use std::rc::Rc;
use crate::application::Application;


pub trait Scene {
    fn on_create(&self) -> ();
    fn on_destroy(&self) -> ();
    fn on_enter(&self) -> ();
    fn on_leave(&self) -> ();
    fn process_input(&self) -> ();
    fn update(&self) -> ();
    fn draw(&self) -> ();
}

pub struct SceneManager {
    scenes: Vec<Rc<dyn Scene>>,
    current_scene: Option<Rc<dyn Scene>>
}

impl SceneManager {
    fn process_input(&self) -> () {

    }
}