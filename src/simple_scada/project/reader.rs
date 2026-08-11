use std::io;
use std::io::Read;
use crate::simple_scada::project::model::{ProjectDate, ProjectInfo, ProjectVersion};

enum State{
    Signature,
    Version {
        signature: u64
    },
    Date {
        signature: u64,
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
                    signature: read_64u(reader)?,
                },

            State::Version { signature } =>
                State::Date{
                    signature,
                    version: read_version(reader)?,
                },

            State::Date { signature, version} =>
                State::Done(ProjectInfo{ signature,
                    version,
                    date: read_date(reader)?,
                }),

            State::Done(project) => return Ok(project)
        };
    }
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

fn read_64u(reader: &mut impl Read) -> io::Result<u64> {
    let mut buffer = [0; 8];
    reader.read_exact(&mut buffer)?;
    Ok(u64::from_le_bytes(buffer))
}