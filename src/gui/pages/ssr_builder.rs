use crate::gui::app::Message;
use crate::gui::components::page_header;
use iced::widget::{Container, column, container, text};
use iced::{Element, Fill};

pub fn view() -> Element<'static, Message>{

    let body : Container<Message> = container(
        text("Функция пока недоступна.")
    )
        .center_x(Fill)
        .center_y(Fill);

    let content = column![
        page_header::view("Формирование SSR файла"),
        body.padding(20).width(Fill),
    ];

    container(content)
        .center_x(Fill)
        .center_y(Fill)
        .padding([8,16])
        .into()
}