mod canvas;
mod controller;
mod model;
mod utils;
mod view;
use tokio;

use std::{process::exit, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    canvas::Canvas,
    controller::{state::State, Controller},
    model::Model,
    utils::api::RestClient,
    view::View,
};

#[tokio::main]
async fn main() {
    let client = match RestClient::new() {
        Some(client) => client,
        None => exit(-1),
    };

    let mut canvas = Canvas::new().expect("");
    let mut view: View = View::new();
    let controller: Controller = Controller::new();

    let model = Arc::new(Mutex::new(Model::new()));
    let model_clone = Arc::clone(&model);

    tokio::spawn(async move {
        let mut lock = model_clone.lock().await;
        match lock.update(&client).await {
            Ok(_) => {}
            Err(error) => {
                println!("{error:#?}");
                return;
            }
        }
    });

    loop {
        let model_clone = model.lock().await.clone();

        canvas
            .draw(|frame| view.render(&model_clone, frame))
            .expect("terminal has failed to draw");

        match controller.run(&model_clone, &mut view.context_mut()) {
            State::Continue => continue,
            _ => break,
        }
    }

    canvas.clear();
}
