use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectEntry{
    pub name : String,
    pub path : PathBuf,
}

impl std::fmt::Display for ProjectEntry{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

#[derive(Debug, Clone)]
pub struct ProjectInfo{
    pub create_at : DelphiDateTime,
    pub version : ProjectVersion,
    pub date : ProjectDate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectVersion{
    pub major : u16,
    pub minor : u16,
    pub patch : u16,
    pub build : u16,
}

impl std::fmt::Display for ProjectVersion{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.major, self.minor, self.patch, self.build)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectDate{
    pub day : u8,
    pub month : u8,
    pub year : u16,
}

impl std::fmt::Display for ProjectDate{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DelphiDateTime{
    raw: f64
}

impl From<f64> for DelphiDateTime{
    fn from(raw: f64) -> Self {
        Self { raw }
    }
}
impl DelphiDateTime{
    pub fn raw(&self) -> f64{
        self.raw
    }
    pub fn to_system_time(&self) -> Option<SystemTime> {
        let seconds = (self.raw - 25569.0) * 86400.0;

        if !seconds.is_finite() {
            return None;
        }

        if seconds >= 0.0 {
            UNIX_EPOCH.checked_add(Duration::from_secs_f64(seconds))
        } else {
            UNIX_EPOCH.checked_sub(Duration::from_secs_f64(-seconds))
        }
    }
}