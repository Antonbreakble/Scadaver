use crate::gui::app::Message;
use iced::widget::{Container, column, container, text};
use iced::{Element, Fill};

pub fn view() -> Element<'static, Message>{

    let body : Container<Message> = container(
        text("Функция пока недоступна.")
    )
        .center_x(Fill)
        .center_y(Fill);

    let content = column![
        body.padding(20).width(Fill),
    ];

    container(content)
        .center_x(Fill)
        .center_y(Fill)
        .padding([8,16])
        .into()
}