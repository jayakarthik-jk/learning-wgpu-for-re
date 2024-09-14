use peniko::{Color, Style};

trait Window {
    fn create();
    fn resize();
}

trait SceneContainer {
    fn draw(); // -> ContainerLayout;
}

trait Scene {
    fn setup();
    fn draw();
}

struct CircleDescriptor {
    x: Size,
    y: Size,
    radius: Size,
    style: Style,
}

struct RectDescriptor {
    x: Size,
    y: Size,
    width: Size,
    height: Size,
    style: Style,
}

struct LineDescriptor {
    x: Size,
    y: Size,
    style: Style,
}

trait Shapes {
    // Basic Shapes
    // Each should have a descriptor with default implementation
    fn point();
    fn line();
    fn circle(desc: CircleDescriptor);
    fn ellipse();
    fn rect(desc: RectDescriptor);
    fn quad();
    fn arc();
    fn triangle();
}

trait Settings {
    // Basic Settings
    fn background();
}

struct Size;
trait SizeImpl {}

struct Stroke;
