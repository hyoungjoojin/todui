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
use std::{sync::Arc, thread, time::Duration};
use tokio::{self, sync::Mutex};

#[tokio::main]
async fn main() {
    let mut canvas = Canvas::new().expect("");
    let mut app = App::new();
    let controller: Controller = Controller::new();

    let model = Arc::new(Mutex::new(Model::new()));
    let model_clone = model.clone();
    tokio::spawn(async move {
        let mut model = model_clone.lock().await;
        match model.update().await {
            Ok(_) => {}
            Err(error) => {
                println!("{error:#?}");
                return;
            }
        }
    });

    loop {
        let model_clone = model.clone();
        let model = model.lock().await;

        canvas
            .draw(|frame| app.render(&model, frame))
            .expect("terminal has failed to draw");

        match controller.run(&model, app.context_mut()) {
            State::Continue => {
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            State::Reload => {
                let model_clone = model_clone.clone();
                tokio::spawn(async move {
                    let mut model = model_clone.lock().await;
                    match model.update().await {
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
