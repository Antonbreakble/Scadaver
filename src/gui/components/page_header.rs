use crate::gui::app::Message;
use iced::widget::{Button, Container, button, container, stack, text};
use iced::{Element, Fill};

pub fn view(title: &str) -> Element<'_, Message> {

    let back_button : Button<Message> = button("Назад")
        .height(Fill)
        .padding([4,8])
        .on_press(Message::Back);

    let title_text : Container<Message> = container(text(title).size(24))
        .center_y(Fill)
        .center_x(Fill);

    stack![
        title_text,
        back_button,
    ]
    .width(Fill)
    .height(32)
    .into()
}