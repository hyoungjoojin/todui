mod canvas;
mod controller;
mod model;
mod utils;
mod view;

use crate::{
    canvas::Canvas,
    controller::{state::State, Controller},
    model::Model,
    utils::security::get_api_token,
    view::View,
};

// #[tokio::main]
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
    let mut model = Model::new();
    let mut view: View = View::new();
    let controller: Controller = Controller::new();

    match model.update(&api_token) {
        Ok(_) => {}
        Err(error) => {
            println!("{error:#?}");
            return;
        }
    }

    loop {
        canvas
            .draw(|frame| view.render(&model, frame))
            .expect("terminal has failed to draw");

        match controller.run(&model, &mut view.context_mut()) {
            State::Continue => continue,
            _ => break,
        }
    }

    canvas.clear();
}
