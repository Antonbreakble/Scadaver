use std::fs::File;
use std::path::PathBuf;
use crate::simple_scada::project::{read_project, ProjectVersion, ProjectDate};

#[test]
fn read_real_project_header(){

    //Чтение файла проекта созданного в версии 7.5.
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/simple_scada/project/tests/data/Project.spr");

    let mut file = File::open(&path).unwrap();

    let project = read_project(&mut file).unwrap();

    let target_version = ProjectVersion {
        major: 2,
        minor: 7,
        patch: 5,
        build: 1,
    };

    let build_time = ProjectDate {
        day: 1,
        month: 4,
        year: 2026,
    };

    assert_eq!(project.version, target_version);
    assert_eq!(project.date, build_time);
    
}