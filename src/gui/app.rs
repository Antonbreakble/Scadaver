use crate::gui::pages;
use iced::Element;

#[derive(Default)]
pub struct Scadaver {
    page : Page,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Page{
    #[default]
    Home,

    ProjectAnalysis,
    SsrBuilder,
    ScriptTransfer,
    Database,
}

#[derive(Debug, Clone)]
pub enum Message{
    Navigate(Page),
    Back,
}

pub fn update(app: &mut Scadaver, message: Message){
    match message {
        Message::Navigate(page) => {
            app.page = page;
        }
        Message::Back => {
            app.page = Page::default();
        }
    }
}

pub fn view(app: &Scadaver) -> Element<'_,Message> {
    match app.page {
        Page::Home => pages::home::view(),
        Page::ProjectAnalysis => pages::project_analysis::view(),
        Page::SsrBuilder => pages::ssr_builder::view(),
        Page::ScriptTransfer => pages::script_transfer::view(),
        Page::Database => pages::database::view(),
    }
}