use iced::{Border, Theme};
use iced::widget::container;

pub fn bordered(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: None,
        border: Border::default()
            .width(1)
            .rounded(6)
            .color(palette.background.strong.color),
        ..Default::default()
    }
}