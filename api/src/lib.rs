use std::fmt::Display;

use peniko::kurbo::{PathSeg, Shape};

pub trait Scene {
    fn on_render(&self, canvas: &mut Canvas);
    fn on_click(&mut self) {}
    // other listeners. they can call rerender in listeners.
}

impl dyn Scene {
    fn rerender(&mut self) {
        // scene will hold the width and height of itself.
        // we need to calculate the actual position of the scene and clear the scene and rerun the
        // on_render function
    }
}

pub struct Canvas<'a> {
    commands: &'a mut Vec<PathSeg>,
}

impl<'a> Canvas<'a> {
    pub fn new(commands: &'a mut Vec<PathSeg>) -> Self {
        Self { commands }
    }

    pub fn draw(&mut self, shape: impl Shape) {
        self.commands.extend(shape.path_segments(1.0));
    }

    // is there any better approach? instead of creating seperate function, try to implement Shape for Scene.
    pub fn draw_scene(&mut self, scene: &impl Scene) {
        scene.on_render(self);
    }
}

#[derive(Default)]
pub struct Engine {
    // is it effecient to store the commands is vec?
    // instead of storing the command in vec. can we try to render it immediately.
    //
    // ???
    // can we create a seperate entity that draws into the texture.
    // let the entity run on seperate thread and use channel to pass this command.
    // is it good?
    // I think with this the scene tree traveral and drawing will become concurrent.
    commands: Vec<PathSeg>,
}

impl Engine {
    pub fn run(&mut self, scene: &mut impl Scene) {
        let mut canvas = Canvas::new(&mut self.commands);
        scene.on_render(&mut canvas);
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.commands)
    }
}
