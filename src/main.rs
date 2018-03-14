#![feature(box_syntax, termination_trait_lib)]

extern crate id3;
extern crate metaflac;
extern crate walkdir;

mod data;

use data::{Artist, Album, Song, Filetype, Tag};
use metaflac::block::{Block, BlockType};
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use walkdir::WalkDir;

fn main() {
    let code = std::process::Termination::report(main_());
    std::process::exit(code);
}

fn main_() -> Result<(), Box<Error>> {
    let path = env::args().skip(1).next().unwrap();

    let iter = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| {
            e.ok().and_then(|e| {
                if e.file_type().is_file() {
                    Some(e)
                } else {
                    None
                }
            })
        });

    let songs = iter.filter_map(|file| {
        let path = file.path();
        let ext  = path.extension().and_then(OsStr::to_str);
        match ext {
            Some("flac") => {
                let meta  = metaflac::Tag::read_from_path(file.path()).ok()?;
                let block = meta.get_blocks(BlockType::VorbisComment).pop()?;
                let meta  = if let &Block::VorbisComment(ref b) = block { &b.comments }
                            else { unreachable!() };

                let mut misc = vec![];
                if let Some(album_artist) = meta.get("ALBUMARTIST") {
                    misc.push(Tag::AlbumArtist(album_artist.first()?.clone()));
                }

                Some(Song {
                    path:     path.into(),
                    filetype: Filetype::FLAC,
                    track:    meta.get("TRACKNUMBER")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.parse().ok()),
                    artist:   meta.get("ARTIST")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.into())
                                  .cloned()
                                  .unwrap_or_default(),
                    title:    meta.get("TITLE")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.into())
                                  .cloned()
                                  .unwrap_or_default(),
                    misc:     misc,
                })
            },
            Some("mp3") => {
                let meta = id3::Tag::read_from_path(file.path()).ok()?;
                Some(Song {
                    path:     path.into(),
                    filetype: Filetype::MP3,
                    track:    meta.track().map(|n| n as u8),
                    artist:   meta.artist().unwrap_or_default().into(),
                    title:    meta.title().unwrap_or_default().into(),
                    misc:     vec![],
                })
            },
            _ => None,
        }
    });

    println!("{:#?}", songs.take(3).collect::<Vec<_>>());

    Ok(())
}
