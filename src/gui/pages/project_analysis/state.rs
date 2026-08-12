use std::path::PathBuf;
use crate::simple_scada::project::{ProjectEntry, ProjectInfo};

#[derive(Default)]
pub struct State{
    pub root_directory : Option<PathBuf>,
    pub projects : Vec<ProjectEntry>,
    pub selected_project: Option<ProjectEntry>,
    pub project_info : Option<ProjectInfo>,
    pub error: Option<String>
}

#[derive(Debug, Clone)]
pub enum Message{
    SelectDirectory,
    DirectorySelected(PathBuf),
    SelectDirectoryCancelled,
    ProjectSelected(ProjectEntry),
    ProjectInfoLoaded(Result<ProjectInfo, String>),
}