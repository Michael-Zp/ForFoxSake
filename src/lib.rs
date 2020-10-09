//This file represents the interface between the wasm and the js
mod utils;
mod for_fox_sake;
mod model;
mod view;
mod view_models;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext};

use image;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern 
{
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() 
{
    alert("Hello, for-fox-sake!");
}

#[wasm_bindgen]
pub struct FoxGame
{
    game: for_fox_sake::ForFoxSake,
}

extern crate web_sys;


#[wasm_bindgen]
impl FoxGame
{
    pub fn new(canvas_id: String, canvas_width: i32, canvas_height: i32, tile_map_raw_data: std::vec::Vec<u8>, sprite_tile_map_raw_data: std::vec::Vec<u8>) -> Result<FoxGame, JsValue>
    {
        utils::set_panic_hook();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        context.viewport(0, 0, canvas_width, canvas_height);
    
        let tile_map = image::load_from_memory_with_format(&tile_map_raw_data, image::ImageFormat::Bmp).unwrap().to_rgba();
        let sprite_tile_map = image::load_from_memory_with_format(&sprite_tile_map_raw_data, image::ImageFormat::Bmp).unwrap().to_rgba();
        let game = for_fox_sake::ForFoxSake::new(context, tile_map, sprite_tile_map)?;


        Ok(FoxGame {
            game: game,
        })
    }

    pub fn update(&mut self, delta_time: f32)
    {
        self.game.update(delta_time);
    }

    pub fn draw(&self) 
    {
        self.game.draw();
    }

    pub fn key_down(&mut self, key_code: i32)
    {
        self.game.key_down(key_code);
    }

    pub fn key_up(&mut self, key_code: i32)
    {
        self.game.key_up(key_code);
    }
}