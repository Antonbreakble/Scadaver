#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod gui;
pub mod simple_scada;

fn main() -> iced::Result {
    gui::run()
}
