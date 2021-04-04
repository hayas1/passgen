use wasm_bindgen::prelude::wasm_bindgen;
use yew::app::App;

use crate::components::addone::Model;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
