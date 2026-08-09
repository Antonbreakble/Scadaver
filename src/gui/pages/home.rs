use crate::gui::app::{Message, Page};
use iced::widget::{
    button,
    column,
    container,
    text,
};
use iced::{Element, Fill};

pub fn view() -> Element<'static, Message> {
    let content = column!
    [
        text("Scadaver")
            .size(42),

        text("Инструменты для Simple-SCADA")
            .size(18),

        button(
            column![
                text("Анализ проекта").size(22),
                text("Анализ структуры проекта").size(14),
            ]
            .spacing(5),
        )
        .width(Fill)
        .height(100)
        .on_press(Message::Navigate(Page::ProjectAnalysis)),

        button(
            column![
                text("Сформировать SSR").size(22),
                text("Пока недоступно").size(14),
            ]
            .spacing(5),
        )
        .width(Fill)
        .height(100)
        .on_press(Message::Navigate(Page::SsrBuilder)),


        button(
            column![
                text("Импорт / экспорт скриптов").size(22),
                text("Пока недоступно").size(14),
            ]
            .spacing(5),
        )
        .width(Fill)
        .height(100)
        .on_press(Message::Navigate(Page::ScriptTransfer)),

        button(
            column![
                text("Управление базой данных").size(22),
                text("Пока недоступно").size(14),
            ]
            .spacing(5),
        )
        .width(Fill)
        .height(100)
        .on_press(Message::Navigate(Page::Database)),
    ].spacing(16).width(550);

    container(content)
        .width(Fill)
        .height(Fill)
        .center_x(Fill)
        .center_y(Fill)
        .into()
}