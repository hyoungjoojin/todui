mod controller;
mod utils;
mod view;

use crate::{
    controller::{state::State, Controller},
    utils::canvas::Canvas,
    view::View,
};

fn main() {
    let mut canvas = Canvas::new();
    let mut view: View = View::new();
    let controller: Controller = Controller::new();

    loop {
        canvas
            .draw(|frame| view.render(frame))
            .expect("terminal has failed to draw");

        match controller.run(&mut view.context) {
            State::Continue => continue,
            _ => break,
        }
    }

    canvas.clear();
}
