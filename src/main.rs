use tray_icon::{
    TrayIconBuilder,
    menu::{Menu}
};

use tao::{
    event_loop::{EventLoop, EventLoopBuilder},
};

const ICON: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/check.png"));

pub enum TrayEvents {}

fn main() {

    let event_loop = setup_event_loop();

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON)
            .expect("Bad icon path")
            .into_rgba8();

        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Failed to load icon");

    let menu = Menu::new();
    let _tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .build()
        .unwrap();

    event_loop.run(move |_event, _event_loop, _control_flow| {});
}

#[cfg(target_os = "macos")]
fn setup_event_loop() -> EventLoop<TrayEvents> {
    use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};

    let mut event_loop = EventLoopBuilder::with_user_event().build();
    event_loop.set_activation_policy(ActivationPolicy::Prohibited);
    event_loop
}
