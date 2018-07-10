use id3;
use metaflac;
use std;
use std::fmt::{self, Display};
use std::path::Path;

#[derive(Debug)]
pub struct Error {
    what: &'static str,
    path: String,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error {
        Error {
            what: "i/o error",
            path: "".into(),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str { "missing some metadata" }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("what", &self.what)
            .field("path",  &self.path)
            .finish()
    }
}

pub fn missing(what: &'static str, path: &Path) -> impl FnOnce() -> Error
{
    let path = path.to_string_lossy().to_string();
    move || Error { what, path }
}

pub fn failed_flac(path: &Path) -> impl FnOnce(metaflac::Error) -> Error
{
    let what = "failed to read flac tags";
    let path = path.to_string_lossy().to_string();
    move |_| Error { what, path }
}

pub fn failed_id3(path: &Path) -> impl FnOnce(id3::Error) -> Error
{
    let what = "failed to read id3 tags";
    let path = path.to_string_lossy().to_string();
    move |_| Error { what, path }
}
