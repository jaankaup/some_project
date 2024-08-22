// use dioxus_desktop::{Config, WindowBuilder, launch_with_props};
use dioxus::prelude::*;
use dioxus::events::*;
use dioxus_desktop::{Config, WindowBuilder, LogicalSize};

pub fn main_page() -> Element {
    rsx! {"jeejee"}
}

fn main() {

    // Main application configuration.
    let config = Config::new().with_window(
        WindowBuilder::default()
        .with_title("Smoke!")
        .with_inner_size(LogicalSize::new(1280.0, 1024.0)),
        );


    // Launch the main application.
    LaunchBuilder::new()
        .with_cfg(desktop!(config))
        .launch(main_page);
}
