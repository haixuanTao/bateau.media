use indexmap::indexmap;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.

    //  val.set_class_name("prompt");
    // let val = document.create_element("a")?;
    // val.set_text_content(Some("> jobs?"));
    // val.set_attribute("href", "www.google.com")?;
    // body.append_child(&val)?;

    // let val = document.create_element("p")?;
    // val.set_text_content(Some("> Blogs?"));
    // body.append_child(&val)?;
    Ok(())
}
