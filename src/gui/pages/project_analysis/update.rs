use std::path::PathBuf;

use iced::Task;

use crate::simple_scada::project::{
    find_projects,
    read_project_info,
    ProjectEntry,
    ProjectInfo,
};

use super::{Message, State};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::SelectDirectory => select_directory(),
        Message::DirectorySelected(path) => {
            directory_selected(state, path)
        }
        Message::SelectDirectoryCancelled => Task::none(),
        Message::ProjectSelected(project) => project_selected(state, project),
        Message::ProjectInfoLoaded(result) => {
            project_info_loaded(state, result)
        }
    }
}

fn select_directory() -> Task<Message> {
    Task::perform(pick_directory(),
                  |path| match path {
                      Some(path) => Message::DirectorySelected(path),
                      None => Message::SelectDirectoryCancelled,
                  })
}

async fn pick_directory() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .set_title("Выберите каталог проектов Simple-Scada 2")
        .pick_folder()
        .await
        .map(|folder| folder.path().to_path_buf())
}

fn directory_selected(state: &mut State, path: PathBuf) -> Task<Message> {
    state.root_directory = Some(path.clone());

    state.projects.clear();
    state.selected_project = None;
    state.error = None;

    match find_projects(&path) {
        Ok(projects) => state.projects = projects,
        Err(error) => {
            state.error = Some(error.to_string());
        }
    };

    Task::none()
}

fn project_selected(state: &mut State, project: ProjectEntry) -> Task<Message> {
    state.selected_project = Some(project.clone());
    state.project_info = None;
    state.error = None;

    Task::perform(
        async move {
            read_project_info(&project.path).map_err(|error| error.to_string())
        },
        Message::ProjectInfoLoaded,
    )

}

fn project_info_loaded(state: &mut State, result: Result<ProjectInfo, String>, ) -> Task<Message> {
    match result {
        Ok(project_info) => {
            state.project_info = Some(project_info);
        }

        Err(error) => {
            state.project_info = None;
            state.error = Some(error);
        }
    }

    Task::none()
}