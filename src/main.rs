use web_sys::{ window } ;

trait Dom {
}

pub struct Button {
}

impl Dom for Button {
}

pub struct Div {
    children: Vec<Box<dyn Dom>>
}

impl Dom for Div {
}

fn main() {
    console_error_panic_hook::set_once();

    let document = window().and_then(|win| win.document()).unwrap();
    let body = document.body().expect("Could not access document.body");

    //let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    // body.append_child(text_node.as_ref()).expect("Failed to append text");


    let root = Div {
        children: vec![Box::new(Button {})]
    };

    body.set_inner_html("FISH");
}
