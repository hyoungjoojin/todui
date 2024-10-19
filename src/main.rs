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
use app::context::editor::EditorStage;
use std::{sync::Arc, time::Duration};
use tokio::{self, sync::Mutex, time::sleep};
use utils::api::HttpMethod;

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
            State::PostTask => {
                let id = app
                    .context_mut()
                    .editor_context()
                    .get_field(EditorStage::ID)
                    .value
                    .clone();

                model
                    .client()
                    .send(
                        format!("/tasks/{}", id.as_str()).as_str(),
                        HttpMethod::POST,
                        Some(app.context_mut().editor_context().build_body()),
                    )
                    .await
                    .unwrap();
            }
            _ => break,
        }
    }

    canvas.clear();
}
