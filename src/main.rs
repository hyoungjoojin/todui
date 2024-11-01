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
use tracing::instrument;
use tracing_subscriber::{fmt::layer, layer::SubscriberExt, util::SubscriberInitExt, Registry};
use utils::api::HttpMethod;
use utils::log::initialize_log_file;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
#[instrument]
async fn main() {
    Registry::default()
        .with(layer().with_ansi(false).with_writer(initialize_log_file()))
        .init();

    tracing::info!(
        "Application [todui v{version}] has been successfully initialized.",
        version = VERSION
    );

    let mut canvas = Canvas::new();
    let mut app = App::new();
    let controller: Controller = Controller::new();

    let model_lock = Arc::new(Mutex::new(Model::new().await));

    loop {
        let model = model_lock.lock().await;

        canvas.draw(|frame| app.render(&model, frame));

        let state = controller.run(&model, &mut app);

        match state {
            State::Continue => {
                sleep(Duration::from_millis(100)).await;
                continue;
            }
            State::Reload => {
                let model_clone_lock = model_lock.clone();
                tokio::spawn(async move {
                    let mut model_clone = model_clone_lock.lock().await;

                    if let Err(error) = model_clone.update().await {
                        tracing::error!(
                            "Model update has failed due to error {error}",
                            error = error
                        );
                    }
                });
            }
            State::PostTask => {
                let id = app
                    .context_mut()
                    .editor_context()
                    .get_field(EditorStage::Id)
                    .value
                    .clone();

                model
                    .client()
                    .send(
                        format!("/tasks/{}", id.as_str()).as_str(),
                        HttpMethod::Post,
                        Some(app.context_mut().editor_context().build_body()),
                    )
                    .await
                    .unwrap();
            }
            _ => break,
        }
    }

    canvas.clear();

    tracing::info!(
        "Application [todui v{version}] has been successfully terminated.",
        version = VERSION
    );
}
