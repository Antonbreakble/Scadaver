use crate::gui::pages;
use crate::gui::components::page_header;
use crate::gui::pages::project_analysis;

use iced::{Element, Fill, Task};
use iced::widget::{container, column};

#[derive(Default)]
pub struct Scadaver {
    page : Page,
    project_analysis : project_analysis::State,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Page{
    #[default]
    Home,
    ProjectAnalysis,
    SscBuilder,
    ScriptTransfer,
    Database,
}

impl Page {
    pub fn title(self) -> &'static str{
        match self {
            Page::Home => "Scadaver",
            Page::ProjectAnalysis => "Анализ проект Simple-Scada 2",
            Page::SscBuilder => "Формирование SSC файла",
            Page::ScriptTransfer => "Экспорт / Импорт скриптов",
            Page::Database => "Управление базами данных",
        }
    }

    pub fn can_go_back(self) -> bool{
        match self {
            Page::Home => false,
            Page::ProjectAnalysis => true,
            Page::SscBuilder => true,
            Page::ScriptTransfer => true,
            Page::Database => true,
        }
    }
}


#[derive(Debug, Clone)]
pub enum Message{
    Navigate(Page),
    Back,

    ProjectAnalysis(project_analysis::Message),
}

pub fn update(app: &mut Scadaver, message: Message)->Task<Message>{
    match message {
        Message::Navigate(page) => {
            app.page = page;
            Task::none()
        }
        Message::Back => {
            app.page = Page::default();
            Task::none()
        }
        Message::ProjectAnalysis(message_from_project_analysis) => {
            project_analysis::update(&mut app.project_analysis, message_from_project_analysis)
                .map(Message::ProjectAnalysis)
        }
    }
}

pub fn view(app: &Scadaver) -> Element<'_,Message> {
    let body = match app.page {
        Page::Home => pages::home::view(),
        Page::ProjectAnalysis =>{
            project_analysis::view(&app.project_analysis).map(Message::ProjectAnalysis)
        },
        Page::SscBuilder => pages::ssc_builder::view(),
        Page::ScriptTransfer => pages::script_transfer::view(),
        Page::Database => pages::database::view(),
    };

    let header = page_header::view(app.page.title(), app.page.can_go_back());

    let content = column![header, body];

    container(content)
        .width(Fill)
        .height(Fill)
        .padding([8, 16])
        .into()
}