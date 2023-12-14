// build.rs

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        ",
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

fn question_html(q: String) -> String {
    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let span = document.create_element("span").unwrap();
    // let val = document.create_element("a").unwrap();
    // val.set_attribute("href", &format!("/?query={q}")).unwrap();
    // val.set_text_content(Some(&q));
    // span.set_class_name("prompt");
    // span.append_child(&val).unwrap();
    // span.inner_html()
    "".to_string()
}

fn create_asset(path: &str) -> Vec<((String, String), (String, String))> {
    let path = PathBuf::from(path);
    let mut vec = Vec::new();
    for folders in path.read_dir().expect("read_dir call failed") {
        let folders = folders.expect("folder failed");
        let folder_path = folders.path();
        for folder in folder_path.read_dir().expect("read dir failed") {
            let folder = folder.expect("folder failed");
            let folder_path = folder.path();
            for file in folder_path.read_dir().expect("read_dir call failed") {
                let file = file.expect("file failed");
                let file_path = file.path().to_str().unwrap();
                vec.push((
                    (folder.file_name(), file.file_name()),
                    (
                        question_html(file.path().to_str().unwrap().to_string()),
                        include_str!(file_path).to_string(),
                    ),
                ))
            }
        }
    }

    vec
}
