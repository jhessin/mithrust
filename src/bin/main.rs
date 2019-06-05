#[macro_use]
extern crate stdweb;

fn main() {
    let message = "Hello, WASM!";
    let message_node = stdweb::web::document().create_text_node(message);
    js! {
        document.body.appendChild(@{message_node})
    }
}
