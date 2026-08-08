use markdown_studio::app::App;

fn main() {
    // Initialize console logging for WASM
    console_log::init_with_level(log::Level::Debug)
        .expect("error initializing logger");

    log::info!("🦀 Markdown Studio starting...");

    leptos::mount::mount_to_body(App);
}
