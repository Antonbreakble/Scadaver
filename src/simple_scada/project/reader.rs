use std::io;
use std::io::Read;
use crate::simple_scada::project::DelphiDateTime;
use crate::simple_scada::project::model::{ProjectDate, ProjectInfo, ProjectVersion};

enum State{
    Signature,
    Version {
        create_at: DelphiDateTime
    },
    Date {
        create_at: DelphiDateTime,
        version: ProjectVersion,
    },
    Done(ProjectInfo),
}

pub fn read_project(reader: &mut impl Read) -> io::Result<ProjectInfo> {
    let mut state = State::Signature;

    loop{
        state = match state {
            State::Signature =>
                State::Version{
                    create_at: read_created_at(reader)?
                },

            State::Version { create_at } =>
                State::Date{
                    create_at,
                    version: read_version(reader)?,
                },

            State::Date { create_at, version} =>
                State::Done(ProjectInfo{
                    create_at,
                    version,
                    date: read_date(reader)?,
                }),

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

fn read_64f(reader: &mut impl Read) -> io::Result<f64> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(f64::from_le_bytes(buffer))
}