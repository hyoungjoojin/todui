mod app;
mod canvas;
mod controller;
mod model;
mod utils;

use crate::{
    app::App,
    canvas::Canvas,
    controller::{state::State, Controller},
    model::Model,
};
use std::{sync::Arc, time::Duration};
use tokio::{self, sync::Mutex, time::sleep};

#[tokio::main]
async fn main() {
    let mut canvas = Canvas::new().expect("");
    let mut app = App::new();
    let controller: Controller = Controller::new();

    let model_lock = Arc::new(Mutex::new(Model::new().await));

    loop {
        let model = model_lock.lock().await;

        canvas
            .draw(|frame| app.render(&model, frame))
            .expect("terminal has failed to draw");

        let state = controller.run(&model, app.context_mut());

        match state {
            State::Continue => {
                sleep(Duration::from_millis(100)).await;
                continue;
            }
            State::Reload => {
                let model_clone_lock = model_lock.clone();
                tokio::spawn(async move {
                    let mut model_clone = model_clone_lock.lock().await;
                    match model_clone.update().await {
                        Ok(_) => {}
                        Err(error) => {
                            println!("{error:#?}");
                            return;
                        }
                    }
                });
            }
            _ => break,
        }
    }

    canvas.clear();
}
