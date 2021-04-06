use wasm_bindgen::prelude::wasm_bindgen;
use yew::app::App;

use crate::components::generator_pane::GeneratorPane;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<GeneratorPane>::new().mount_to_body();
}
