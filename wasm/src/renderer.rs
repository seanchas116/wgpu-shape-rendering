use log::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Renderer {
    canvas: web_sys::HtmlCanvasElement,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Renderer {
        info!("renderer");
        Renderer { canvas }
    }
}
