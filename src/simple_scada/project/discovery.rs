use std::io;
use std::path::Path;
use crate::simple_scada::project::model::{ProjectEntry};

pub fn find_projects(directory: &Path) -> io::Result<Vec<ProjectEntry>> {
    let mut projects = Vec::new();

    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        if !path.join("Project.spr").is_file(){
            continue;
        }

        let Some(name) = path.file_name().and_then(|s| s.to_str()) else { continue };

        projects.push(ProjectEntry{
            name: name.to_owned(),
            path,
        })
    }
    Ok(projects)
}