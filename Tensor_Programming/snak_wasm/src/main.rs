#[macro_use]
extern crate stdweb;

mod canvas;
mod direction;
mod snake;

use canvas::Canvas;

fn main() {
    stdweb::initialize();

    let canvas= Canvas::new("#canvas", 20, 20);

    canvas.draw(5, 5, "red");
    canvas.draw(10, 10, "orange");
    canvas.draw(15, 10, "blue");

    stdweb::event_loop();
}
