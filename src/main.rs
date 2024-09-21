mod canvas;
mod controller;
mod view;

use crate::{
    canvas::Canvas,
    controller::{state::State, Controller},
    view::View,
};

fn main() {
    let mut canvas = Canvas::new().expect("");
    let mut view: View = View::new();
    let controller: Controller = Controller::new();

    loop {
        canvas
            .draw(|frame| view.render(frame))
            .expect("terminal has failed to draw");

        match controller.run(&mut view.context()) {
            State::Continue => continue,
            _ => break,
        }
    }

    canvas.clear();
}
