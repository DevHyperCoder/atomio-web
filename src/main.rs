#[allow(clippy::excessive_precision)]
mod data;

mod app;
mod input;
mod stats;
mod utils;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
