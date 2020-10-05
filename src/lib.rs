//This file represents the interface between the wasm and the js

mod utils;
mod for_fox_sake;
mod model;
mod view;

use wasm_bindgen::prelude::*;

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
pub fn startGame() -> Result<(), JsValue> 
{
    let a = for_fox_sake::ForFoxSake::new()?;

    a.test_interface();

    Ok(())
}