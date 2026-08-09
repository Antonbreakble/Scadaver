use iced::widget::{button, text};
use crate::gui::app::Message;

pub fn view<'a>(
    title: &'a str,
    description: &'a str,
    on_press: Option<Message>) -> iced::Element<'a, Message>{

    let is_enable = on_press.is_some();
    let trailing = if is_enable {
        text("→").size(22)
    } else{
        text("Недоступно").size(14)
    };

    button("").into()
}