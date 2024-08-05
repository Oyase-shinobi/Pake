use crate::app::config::PakeConfig;
use std::path::PathBuf;
use tauri::{App, Window, WindowBuilder, WindowUrl};

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

pub fn get_window(app: &mut App, config: PakeConfig, _data_dir: PathBuf) -> Window {
    let window_config = config
        .windows
        .first()
        .expect("At least one window configuration is required");

    let user_agent = config.user_agent.get();

    let url = match window_config.url_type.as_str() {
        "web" => WindowUrl::App(window_config.url.parse().unwrap()),
        "local" => WindowUrl::App(PathBuf::from(&window_config.url)),
        _ => panic!("url type can only be web or local"),
    };

    let config_script = format!(
        "window.pakeConfig = {}",
        serde_json::to_string(&window_config).unwrap()
    );

    let mut window_builder = WindowBuilder::new(app, "pake", url)
        .title("")
        .user_agent(user_agent)
        .visible(false) // Prevent initial shaking
        .resizable(window_config.resizable)
        .fullscreen(window_config.fullscreen)
        .inner_size(window_config.width, window_config.height)
        .disable_file_drop_handler()
        .always_on_top(window_config.always_on_top)
        .initialization_script(&config_script)
        .initialization_script(include_str!("../inject/component.js"))
        .initialization_script(include_str!("../inject/event.js"))
        .initialization_script(include_str!("../inject/style.js"))
        //This is necessary to allow for file injection by external developers for customization purposes.
        .initialization_script(include_str!("../inject/custom.js"));

    #[cfg(target_os = "macos")]
    {
        let title_bar_style = if window_config.hide_title_bar {
            TitleBarStyle::Overlay
        } else {
            TitleBarStyle::Visible
        };
        window_builder = window_builder.title_bar_style(title_bar_style)
    }

    #[cfg(not(target_os = "macos"))]
    {
        window_builder = window_builder
            .data_directory(_data_dir)
            .title(app.package_info().name)
    }

    window_builder.build().expect("Failed to build window")
}
