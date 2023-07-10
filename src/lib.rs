use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    bind_buttons();
}

fn bind_buttons() {
    let window = web_sys::window().expect("A global `window` must exist");
    let document = window.document().expect("A document must be on window");
    let body = document.body().expect("A document must have a body");

    bind_button(&document, &body, "First", move || on_click_first_button());
    bind_button(&document, &body, "Second", move || on_click_second_button());
}

fn bind_button<T>(document: &Document, body: &HtmlElement, text: &str, handler: T)
where
    T: 'static + Fn() -> (),
{
    let button = document.create_element("button").unwrap();
    let on_click_button: Closure<dyn Fn()> = Closure::new(handler);

    button.set_text_content(Some(text));
    button
        .add_event_listener_with_callback("click", on_click_button.as_ref().unchecked_ref())
        .unwrap();
    on_click_button.forget();

    body.append_child(&button).unwrap();
}

fn on_click_first_button() {
    alert("first button is clicked");
}

fn on_click_second_button() {
    alert("second button is clicked");
}
