use iced::widget::{Container, button, container, stack, text};
use iced::{Element, Fill};

use crate::gui::app::Message;
use crate::gui::styles::button::subtle_bordered;

pub fn view(title: &str, can_go_back : bool) -> Element<'_, Message> {

    let title_text : Container<Message> = container(text(title).size(24))
        .center_y(Fill)
        .center_x(Fill);

    let mut header = stack![title_text];

    if can_go_back {
        header = header.push(
            button("Назад")
                .height(Fill)
                .padding([4, 8])
                .style(subtle_bordered)
                .on_press(Message::Back),
        );
    }

    header.width(Fill).height(32).into()
}