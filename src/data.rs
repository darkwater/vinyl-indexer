use std::path::PathBuf;

#[derive(Debug)]
pub struct Artist {
    pub name:   String,
    pub albums: Vec<Album>,
}

#[derive(Debug)]
pub struct Album {
    pub path:   PathBuf,
    pub artist: String,
    pub name:   String,
    pub songs:  Vec<Song>,
}

#[derive(Debug)]
pub struct Song {
    pub path:         PathBuf,
    pub filetype:     Filetype,
    pub disc:         Option<u8>,
    pub track:        Option<u8>,
    pub artist:       String,
    pub album:        String,
    pub album_artist: Option<String>,
    pub title:        String,
}

#[derive(Debug)]
pub enum Filetype {
    FLAC,
    MP3,
}
