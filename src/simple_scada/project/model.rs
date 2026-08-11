use std::path::PathBuf;

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
    pub signature : u64,
    pub version : ProjectVersion,
    pub date : ProjectDate,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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