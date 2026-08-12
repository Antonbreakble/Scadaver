use crate::gui::app::{Message, Page};
use crate::gui::components::tool_card;
use iced::widget::{
    column,
    container,
    text,
};
use iced::{Element, Fill};

pub fn view() -> Element<'static, Message> {
    let content = column!
    [
        text("Инструменты для Simple-SCADA").size(18),

        tool_card::view(
            "Анализ проекта",
            "Просмотр структуры и содержания проекта",
            Some(Message::Navigate(Page::ProjectAnalysis))
        ),

        tool_card::view(
            "Работа с SSC",
            "Анализ и работа с файлом скриптов",
            None,
        ),

        tool_card::view(
            "Импорт / экспорт скриптов",
            "Импорт / экспорт скриптов из одного проекта в другой",
            None,
        ),

        tool_card::view(
            "Работа с базой данных",
            "Работа с базой данных",
            None,
        ),

    ]
    .spacing(16)
    .width(550);

    container(content)
        .width(Fill)
        .height(Fill)
        .center_x(Fill)
        .center_y(Fill)
        .into()
}