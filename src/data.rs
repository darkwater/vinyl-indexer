use std::path::PathBuf;

#[derive(Debug)]
pub struct Artist {
    pub name:   String,
    pub albums: Vec<Album>,
}

#[derive(Debug)]
pub struct Album {
    pub name:  String,
    pub songs: Vec<Song>,
}

#[derive(Debug)]
pub struct Song {
    pub path:     PathBuf,
    pub filetype: Filetype,
    pub track:    Option<u8>,
    pub artist:   String,
    pub title:    String,
    pub misc:     Vec<Tag>
}

#[derive(Debug)]
pub enum Filetype {
    FLAC,
    MP3,
}

#[derive(Debug)]
pub enum Tag {
    AlbumArtist(String),
}
