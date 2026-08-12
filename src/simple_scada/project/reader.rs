use std::io;
use std::io::Read;
use crate::simple_scada::project::DelphiDateTime;
use crate::simple_scada::project::model::{ProjectDate, ProjectInfo, ProjectVersion};

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
                read_16u(reader)?;

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
    let created_at = read_64f(reader)?;
    Ok(DelphiDateTime::from(created_at))
}

fn read_version(reader: &mut impl Read) -> io::Result<ProjectVersion> {
    Ok(ProjectVersion{
        major: read_16u(reader)?,
        minor: read_16u(reader)?,
        patch: read_16u(reader)?,
        build: read_16u(reader)?,
    })
}

fn read_date(reader: &mut impl Read) -> io::Result<ProjectDate> {
    Ok(ProjectDate{
        day: read_8u(reader)?,
        month: read_8u(reader)?,
        year: read_16u(reader)?,
    })
}

fn read_version_code(reader: &mut impl Read) -> io::Result<u32> {
    Ok(read_32u(reader)?)
}

fn read_sized_string(reader: &mut impl Read) -> io::Result<String> {
    let mut len_buf = [0; 4];
    reader.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;

    let mut buf = vec![0; len];
    reader.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf).unwrap())
}

fn read_8u(reader: &mut impl Read) -> io::Result<u8> {
    let mut buffer = [0; 1];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

fn read_16u(reader: &mut impl Read) -> io::Result<u16> {
    let mut buffer = [0; 2];
    reader.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

fn read_32u(reader: &mut impl Read) -> io::Result<u32> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_64f(reader: &mut impl Read) -> io::Result<f64> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(f64::from_le_bytes(buffer))
}