#[macro_use]
extern crate stdweb;

fn main() {
    let message = "Hello, WASM!";
    js!{
        alert( @{message} );
    };
}
