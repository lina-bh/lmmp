use glutin::dpi::PhysicalSize;
use imgui::{im_str, Condition, StyleVar, Ui, Window};

pub fn toolbar(ui: &Ui, window_size: PhysicalSize<u32>) -> f32 {
    let window_padding = ui.push_style_var(StyleVar::WindowPadding([2f32, 2f32]));
    let frame_padding = ui.push_style_var(StyleVar::FramePadding([0f32, 0f32]));
    let min_size = ui.push_style_var(StyleVar::WindowMinSize([window_size.width as f32, 0f32]));

    let toolbar = Window::new(im_str!("toolbar"))
        .position([0f32, 0f32], Condition::Always)
        .size([window_size.width as f32, 0f32], Condition::Always)
        .no_decoration()
        .begin(&ui)
        .unwrap();
    ui.text("lmmp");
    ui.same_line(0f32);
    ui.button(im_str!("play"), [0f32, 0f32]);
    let sz = ui.window_size();
    toolbar.end(&ui);

    window_padding.pop(&ui);
    frame_padding.pop(&ui);
    min_size.pop(&ui);

    sz[1]
}

pub fn statusbar(ui: &Ui, window_size: PhysicalSize<u32>, height: f32) {
    let window_padding = ui.push_style_var(StyleVar::WindowPadding([2f32, 2f32]));
    let frame_padding = ui.push_style_var(StyleVar::FramePadding([0f32, 0f32]));
    let min_size = ui.push_style_var(StyleVar::WindowMinSize([window_size.width as f32, 0f32]));

    let statusbar = Window::new(im_str!("statusbar"))
        .position(
            [0f32, (window_size.height as f32) - height],
            Condition::Always,
        )
        .size([window_size.width as f32, height], Condition::Always)
        .no_decoration()
        .begin(&ui)
        .unwrap();
    ui.text("statusbar");

    statusbar.end(&ui);

    window_padding.pop(&ui);
    frame_padding.pop(&ui);
    min_size.pop(&ui);
}
