use iced::widget::{
    button,
    column,
    container,
    pick_list,
    row,
    text,
    text_input,
};
use iced::{Element, Fill};

use crate::gui::styles::button::subtle_bordered;
use crate::gui::styles::container::bordered;

use super::{Message, State};

pub fn view(state: &State) -> Element<'_, Message>{

    let mut content = column![
        directory_selector(state),
    ].spacing(10);

    if state.root_directory.is_some() {
        content = content.push(project_selector(state));
    }

    if state.project_info.is_some() {
        content = content.push(
            project_info_card(state)
        );
    }

    container(content)
        .width(Fill)
        .padding([24, 0])
        .center_x(Fill)
        .into()

}

fn directory_selector(state: &State) -> Element<'_, Message> {
    let directory = state.root_directory
        .as_ref()
        .map(|path|path.display().to_string())
        .unwrap_or_else(|| "Каталог не выбран".to_string());

    let directory_selector = column![
        text("Выберите директорию с проектами Simple-Scada 2").size(20),
        row![
            text_input("Путь к директории...",&directory).width(Fill),
            button("Выбрать...").style(subtle_bordered).on_press(Message::SelectDirectory),
        ].spacing(10)
    ]
        .spacing(10);

    container(directory_selector)
        .width(Fill)
        .padding(20)
        .style(bordered)
        .into()
}

fn project_selector(state: &State) -> Element<'_, Message> {
    let content = column![
        text("Выберите проект").size(20),
        row![
            pick_list(
                state.projects.as_slice(),
                state.selected_project.as_ref(),
                Message::ProjectSelected,
            )
            .placeholder("Проект не выбран")
            .width(Fill),
        ].spacing(10)
    ].spacing(10);

    container(content)
        .width(Fill)
        .padding(20)
        .style(bordered)
        .into()
}



fn project_info_card(state: &State) -> Element<'_, Message> {
    let Some(info) = &state.project_info else {
        return container("").into();
    };

    let version_code = info
        .version_code
        .map(|code| code.to_string())
        .unwrap_or_else(|| "Отсутствует".to_string());

    let content = column![
        text("Информация о проекте").size(20),

        info_row("Название:",info.project_name.clone()),
        info_row("Дата создания:",info.create_at.to_string(),),
        info_row("Версия:",info.version.to_string()),
        info_row("Дата версии:",info.date.to_string()),
        info_row("Version code:",version_code),
    ].spacing(10);

    container(content)
        .width(Fill)
        .padding(20)
        .style(bordered)
        .into()
}

fn info_row(name: &'static str, value: String, ) -> Element<'static, Message> {
    row![container(text(name)).width(140),
        text(value),
    ]
        .spacing(10)
        .into()
}