use std::ffi::OsString;

#[derive(Debug)]
pub struct Folder {
    pub path:  OsString,
    pub files: Vec<File>,
}

#[derive(Debug)]
pub struct File {
    pub name:         OsString,
    pub format:       Format,
    pub title:        String,
    pub artist:       String,
    pub album:        String,
    pub album_artist: String,
    pub year:         i64,
    pub disc:         i64,
    pub total_discs:  i64,
    pub track:        i64,
    pub total_tracks: i64,
}

#[derive(Debug)]
pub enum Format {
    FLAC,
    MP3,
}

impl Format {
    pub fn to_str(&self) -> &'static str {
        match self {
            Format::FLAC => "flac",
            Format::MP3  => "mp3",
        }
    }
}
