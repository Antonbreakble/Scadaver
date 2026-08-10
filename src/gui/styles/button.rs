use iced::widget::button;
use iced::{Border, Theme};

pub fn subtle_bordered(theme: &Theme, status: button::Status, ) -> button::Style {
    let palette = theme.extended_palette();

    let mut style = button::subtle(theme, status);

    let border_color = match status {
        button::Status::Hovered => palette.primary.base.color,
        button::Status::Disabled => palette.background.strong.color.scale_alpha(0.4),
        _ => palette.background.strong.color,
    };

    style.border = Border::default()
        .color(border_color)
        .width(1.0)
        .rounded(6);

    style
}