pub mod components;
pub mod pages;
pub mod app;

pub fn run() -> iced::Result{
    iced::application(
        app::Scadaver::default,
        app::update,
        app::view,
    )
        .title("Scadaver")
        .window_size((900, 700))
        .centered()
        .run()
}