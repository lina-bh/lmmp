#[allow(unused_mut)]
#[allow(bare_trait_objects)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
mod window;

fn main() {
    let window = window::LmmpWindow::new();
    window.run();
}
