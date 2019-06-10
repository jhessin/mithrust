#[macro_use]
extern crate stdweb;

pub fn m<'a>(el: &str, text: &'a str) {
    let message_node = stdweb::web::document().create_text_node(text);
    js! {
        document.body.appendChild(@{message_node})
    }
}
