use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::{Path};
use crate::simple_scada::project::DelphiDateTime;
use crate::simple_scada::project::model::{ProjectDate, ProjectInfo, ProjectVersion};
use crate::simple_scada::binary_reader::{
    read_u8,
    read_u16,
    read_u32,
    read_f64,
    read_sized_string,
};

enum State{
    CreatedAt,
    Version {
        create_at: DelphiDateTime
    },
    Date {
        create_at: DelphiDateTime,
        version: ProjectVersion,
    },
    VersionCode{
        create_at: DelphiDateTime,
        version: ProjectVersion,
        date: ProjectDate,
    },
    ProjectName{
        create_at: DelphiDateTime,
        version: ProjectVersion,
        date: ProjectDate,
        version_code: Option<u32>,
    },
    Done(ProjectInfo),
}

pub fn read_project_info(project_path: &Path, ) -> io::Result<ProjectInfo> {
    let file = File::open(project_path.join("Project.spr"))?;
    let mut reader = BufReader::new(file);
    read_project(&mut reader)
}

pub fn read_project(reader: &mut impl Read) -> io::Result<ProjectInfo> {
    let mut state = State::CreatedAt;

    loop{
        state = match state {
            State::CreatedAt =>
                State::Version{
                    create_at: read_created_at(reader)?
                },

            State::Version { create_at } =>
                State::Date{
                    create_at,
                    version: read_version(reader)?,
                },

            State::Date { create_at, version} => {
                let date = read_date(reader)?;
                State::VersionCode {
                    create_at,
                    version,
                    date
                }
            }

            State::VersionCode { create_at, version, date} => {
                let version_code =
                    if version >= ProjectVersion::new(2,7,5,1){
                        Some(read_version_code(reader)?)
                    } else{
                        None
                    };
                State::ProjectName {
                    create_at,
                    version,
                    date,
                    version_code
                }
            }

            State::ProjectName{ create_at, version, date, version_code} => {
                // Пока неизвестная переменная. Поэтому просто читаем и никуда не пишем
                read_u16(reader)?;

                let project_name = read_sized_string(reader)?;
                State::Done(
                    ProjectInfo{
                        create_at,
                        version,
                        date,
                        version_code,
                        project_name,
                    }
                )

            }
            State::Done(project) => return Ok(project)
        };
    }
}

fn read_created_at(reader: &mut impl Read) -> io::Result<DelphiDateTime> {
    let created_at = read_f64(reader)?;
    Ok(DelphiDateTime::from(created_at))
}

fn read_version(reader: &mut impl Read) -> io::Result<ProjectVersion> {
    Ok(ProjectVersion{
        major: read_u16(reader)?,
        minor: read_u16(reader)?,
        patch: read_u16(reader)?,
        build: read_u16(reader)?,
    })
}

fn read_date(reader: &mut impl Read) -> io::Result<ProjectDate> {
    Ok(ProjectDate{
        day: read_u8(reader)?,
        month: read_u8(reader)?,
        year: read_u16(reader)?,
    })
}

fn read_version_code(reader: &mut impl Read) -> io::Result<u32> {
    Ok(read_u32(reader)?)
}