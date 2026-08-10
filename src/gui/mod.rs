pub mod components;
pub mod pages;
pub mod app;
pub mod styles;

use iced::window;

pub fn run() -> iced::Result{

    let icon = window::icon::from_file_data(
        include_bytes!("../../assets/logo.ico"),
        None,
    ).expect("Failed to load icon");

    iced::application(
        app::Scadaver::default,
        app::update,
        app::view,
    )
    .title("Scadaver")
        .window(window::Settings{
            icon: Some(icon),
            ..Default::default()
        })
    .window_size((900, 700))
    .centered()
    .run()
}