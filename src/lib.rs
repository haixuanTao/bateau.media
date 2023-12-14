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
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let url = web_sys::Url::new(&document.url().expect("document should have a url"))
        .expect("Should be parsable");
    let query = url.search_params().get("q");

    let qa = indexmap! {
        "> What is 1ms.ai?" => "1ms.ai aims at building fast AI that can auto-generate its code!",
        "> Current project" => "Current project includes dora-rs and wonnx",
        "> Joining 1ms.ai" => "We have the following opened positions: Internship and Apprenticeship",
    };
    match query {
        None => {
            for question in qa.keys() {
                let div = document.create_element("div")?;
                let span = document.create_element("span")?;
                let val = document.create_element("a")?;
                val.set_attribute("href", format!("/?q={question}").as_str())?;
                val.set_text_content(Some(question));
                span.set_class_name("prompt");
                span.append_child(&val)?;
                div.append_child(&span)?;
                body.append_child(&div)?;
            }
        }
        Some(q) => {
            let span = document.create_element("span")?;
            let val = document.create_element("a")?;
            val.set_attribute("href", &format!("/?q={q}"))?;
            val.set_text_content(Some(&q));
            span.set_class_name("prompt");
            span.append_child(&val)?;
            body.append_child(&span)?;

            let val = document.create_element("p")?;
            val.set_text_content(qa.get(q.as_str()).map(|s| (*s)));
            body.append_child(&val)?;

            let span = document.create_element("span")?;
            let val = document.create_element("a")?;
            val.set_attribute("href", &format!("/"))?;
            val.set_text_content(Some("< Go back"));
            span.set_class_name("prompt");
            span.append_child(&val)?;
            body.append_child(&span)?;
        }
    }

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
