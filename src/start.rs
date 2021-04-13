use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

use super::components::app::PassGenApp;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<PassGenApp>::new().mount_to_body();
}
