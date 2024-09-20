use peniko::kurbo::{Circle, Point, Rect, Size};
use re_api::{Canvas, Engine, Scene};

fn main() {
    // core::run();
    let mut engine = Engine::default();
    engine.run(&mut CustomScene);
    println!("{engine}")
}

struct CustomScene;

impl Scene for CustomScene {
    fn on_render(&self, canvas: &mut Canvas) {
        let circle = Circle::new(Point::new(0.0, 0.0), 1.0);
        canvas.draw(circle);
        let rect = Rect::from_origin_size(Point::new(0.0, 0.0), Size::new(100.0, 100.0));
        canvas.draw(rect);
    }
}
