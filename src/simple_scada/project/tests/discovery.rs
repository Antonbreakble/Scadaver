use std::fs;
use crate::simple_scada::project::find_projects;

#[test]
fn finds_project(){
    let root = tempfile::tempdir().unwrap();
    let project1 = root.path().join("project1");
    let project2 = root.path().join("project2");

    fs::create_dir(&project1).unwrap();
    fs::create_dir(&project2).unwrap();

    fs::File::create(project1.join("Project.spr")).unwrap();
    fs::File::create(project2.join("Project.spr")).unwrap();

    let projects = find_projects(root.path()).unwrap();
    assert_eq!(projects.len(), 2);
    assert_eq!(projects[0].name, "project1");
    assert_eq!(projects[1].name, "project2");
}