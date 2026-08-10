use iced::{Alignment, Fill};
use iced::widget::{button, text, row, column};

use crate::gui::app::Message;
use crate::gui::styles::button::subtle_bordered;

pub fn view<'a>(
    title: &'a str,
    description: &'a str,
    on_press: Option<Message>) -> iced::Element<'a, Message>{

    let is_enable = on_press.is_some();
    let trailing : iced::Element<'a, Message> = if is_enable {
        text("→").size(22).into()
    } else{
        text("Недоступно").size(14).into()
    };

    let content = row![
        column![
            text(title).size(22),
            text(description).size(14),
        ]
        .spacing(8)
        .width(Fill),

        trailing
    ]
    .align_y(Alignment::Center);


    button(content)
        .width(Fill)
        .height(90)
        .padding([14, 16])
        .on_press_maybe(on_press)
        .style(subtle_bordered)
        .into()
}
