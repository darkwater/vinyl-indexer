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
    pub year:         i32,
    pub disc:         i32,
    pub total_discs:  i32,
    pub track:        i32,
    pub total_tracks: i32,
}

#[derive(Debug)]
pub enum Format {
    FLAC,
    MP3,
}
