use crate::index::IndexType;
use core::fmt;
use std::path::PathBuf;
use std::{error, io};

#[derive(Debug)]
pub struct FileSystemError {
    kind: FileSystemErrorKind,
}

#[derive(Debug)]
enum FileSystemErrorKind {
    Msg(String),
    Io(io::Error),
    MainCacheFileNotFound(io::Error, PathBuf),
    IndexNotFound(IndexType),
    IndexEntryNotFound(u32),
}

impl FileSystemError {
    fn new(kind: FileSystemErrorKind) -> Self {
        FileSystemError { kind }
    }
    pub fn msg(value: impl ToString) -> Self {
        Self {
            kind: FileSystemErrorKind::Msg(value.to_string()),
        }
    }
    pub(crate) fn io(ioerr: io::Error) -> Self {
        Self::new(FileSystemErrorKind::Io(ioerr))
    }

    pub(crate) fn main_cache_file_not_found(ioerr: io::Error, path_buf: PathBuf) -> Self {
        Self::new(FileSystemErrorKind::MainCacheFileNotFound(ioerr, path_buf))
    }

    pub(crate) fn index_not_found(index_type: IndexType) -> Self {
        Self::new(FileSystemErrorKind::IndexNotFound(index_type))
    }
    pub(crate) fn index_entry_not_found(entry_id: u32) -> Self {
        Self::new(FileSystemErrorKind::IndexEntryNotFound(entry_id))
    }
}

impl error::Error for FileSystemError {
    fn description(&self) -> &str {
        "file system error"
    }
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            FileSystemErrorKind::Msg(ref message) => write!(f, "{}", message),
            FileSystemErrorKind::Io(ref e) => e.fmt(f),
            FileSystemErrorKind::MainCacheFileNotFound(ref _e, ref path_buf) => {
                write!(f, "{:#?}", path_buf)
            }
            FileSystemErrorKind::IndexNotFound(ref index_type) => {
                write!(f, "Index {} not found.", index_type.id())
            }
            FileSystemErrorKind::IndexEntryNotFound(ref entry_id) => {
                write!(f, "Index Entry {} was not found.", entry_id)
            }
        }
    }
}

impl From<io::Error> for FileSystemError {
    fn from(ioerr: io::Error) -> FileSystemError {
        FileSystemError {
            kind: FileSystemErrorKind::Io(ioerr),
        }
    }
}