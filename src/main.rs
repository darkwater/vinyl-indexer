#![feature(box_syntax, conservative_impl_trait, termination_trait)]

extern crate id3;
extern crate itertools;
extern crate metaflac;
extern crate walkdir;

mod data;
mod error;

use data::{Artist, Album, Song, Filetype};
use error::Error;
use itertools::Itertools;
use metaflac::block::{Block, BlockType};
use std::convert::Into;
use std::env;
use std::ffi::OsStr;
use walkdir::WalkDir;

fn main() -> Result<(), Error> {
    let path = env::args().skip(1).next().unwrap();

    // Walk recursively through the given directory
    let walker = WalkDir::new(path).follow_links(true);

    // Sort listings alphabetically, and group subdirectories first
    let sorted = walker.sort_by(|a, b| {
        a.file_type().is_file().cmp(&b.file_type().is_file())
            .then(a.path().cmp(b.path()))
    });

    // Filter out the directories themselves (only iterate through files)
    let files = sorted.into_iter().filter_map(|file| {
        file.ok().and_then(|file| {
            if file.file_type().is_file() {
                match file.path().extension().and_then(OsStr::to_str) {
                    Some("flac") | Some("mp3") => Some(file),
                    _                          => None,
                }
            } else {
                None
            }
        })
    });

    // Convert DirEntries to Result<Songs>
    let results = files.map(|file| {
        let path = file.path();
        let ext  = path.extension().and_then(OsStr::to_str);
        match ext {
            Some("flac") => {
                let meta = metaflac::Tag::read_from_path(path).map_err(error::failed_flac(path))?;
                let block = meta.get_blocks(BlockType::VorbisComment).pop().ok_or_else(error::missing("metadata", path))?;
                let meta  = if let &Block::VorbisComment(ref b) = block { &b.comments }
                            else { unreachable!() };

                Ok(Song {
                    path:         path.into(),
                    filetype:     Filetype::FLAC,
                    track:        meta.get("TRACKNUMBER")
                                      .and_then(|n| n.first())
                                      .and_then(|n| n.parse().ok()),
                    artist:       meta.get("ARTIST")
                                      .and_then(|n| n.first())
                                      .and_then(|n| n.into())
                                      .cloned().ok_or_else(error::missing("artist", path))?,
                    album:        meta.get("ALBUM")
                                      .and_then(|n| n.first())
                                      .and_then(|n| n.into())
                                      .cloned()
                                      .unwrap_or_else(|| "(Other)".into()),
                    album_artist: meta.get("ALBUMARTIST")
                                      .and_then(|n| n.first())
                                      .and_then(|n| n.into())
                                      .cloned(),
                    title:        meta.get("TITLE")
                                      .and_then(|n| n.first())
                                      .and_then(|n| n.into())
                                      .cloned().ok_or_else(error::missing("title", path))?,
                })
            },
            Some("mp3") => {
                let meta = id3::Tag::read_from_path(path).map_err(error::failed_id3(path))?;
                Ok(Song {
                    path:         path.into(),
                    filetype:     Filetype::MP3,
                    track:        meta.track().map(|n| n as u8),
                    artist:       meta.artist().ok_or_else(error::missing("artist", path))?.into(),
                    album:        meta.album().as_ref().cloned().unwrap_or("(Other)").into(),
                    album_artist: meta.album_artist().as_ref().cloned().map(Into::into),
                    title:        meta.title().ok_or_else(error::missing("title", path))?.into(),
                })
            },
            _ => unreachable!(),
        }
    });

    let (songs, errors): (Vec<_>, Vec<_>) = results.partition(|result: &Result<Song, Error>| {
        result.is_ok()
    });

    let songs  = songs.into_iter().map(|s| s.unwrap());
    let errors = errors.into_iter().map(|s| s.unwrap_err());

    let albums = songs.group_by(|song| {
        song.album.clone()
    });

    let albums = albums.into_iter().map(|(name, songs)| {
        let songs  = songs.collect::<Vec<_>>();
        let path   = songs.first().unwrap().path.parent().unwrap().into();
        let artist = songs.first().unwrap().artist.clone();
        Album {
            path, artist, name, songs
        }
    });

    for album in albums {
        println!("{} - {} [{} songs]", album.artist, album.name, album.songs.len());
    }

    println!("{} errors occurred.", errors.len());

    Ok(())
}
