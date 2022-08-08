use std::rc::Rc;
use std::collections::HashMap;
use crate::application::Application;


pub trait Scene {
    fn on_create(&self) -> ();
    fn on_destroy(&self) -> ();
    fn on_enter(&self) -> ();
    fn on_leave(&self) -> ();
    fn process_input(&self) -> ();
    fn update(&self) -> ();
    fn draw(&self, app: Application) -> ();
}

pub struct SceneManager {
    scenes: HashMap<u32, Rc<dyn scene>>,
    current_scene: Rc<dyn Scene>,
    id: u32
}

impl SceneManager {
    pub fn process_input(&self) -> () {
        self.current_scene.process_input();
    }

    pub fn update(&self) -> () {
        self.current_scene.update();
    }

    pub fn draw(&self, app: Application) -> () {
        self.current_scene.draw(app);
    }

    pub fn add_scene(&mut self, scene: Rc<dyn Scene>) -> u32 {
        self.scenes.insert(self.id, scene);
        scene.on_create();
        self.id = self.id + 1;
        self.id - 1
    }

    // I apologise for how shit this code is. It is temporary.
    pub fn remove(&mut self, id: u32) -> () {
        for (k, v) in self.scenes.iter() {
            if k == id {
                if let Some(scene) = self.scenes.get(&k) {
                    let scene = scene as Rc<dyn Scene>;
                    scene.on_destroy();
                }
            }
        }
        self.scenes.retain(|k, _| k != id);
    }


}