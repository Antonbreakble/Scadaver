use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectEntry{
    pub name : String,
    pub path : PathBuf,
}