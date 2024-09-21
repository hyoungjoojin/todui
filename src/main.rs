mod canvas;
mod controller;
mod utils;
mod view;

use crate::{
    canvas::Canvas,
    controller::{state::State, Controller},
    utils::security::get_api_token,
    view::View,
};

fn main() {
    let api_token = match get_api_token() {
        Ok(token) => token,
        Err(_) => {
            println!("Your Todoist API token could not be found by todui.\n");
            println!("Export or set an environment variable.");
            println!("  export TODOIST_API_TOKEN=\"YOUR_API_TOKEN\"");
            return;
        }
    };

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
