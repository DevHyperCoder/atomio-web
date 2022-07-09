#[allow(clippy::excessive_precision)]
mod data;

mod app;
mod input;
mod utils;
mod stats;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
