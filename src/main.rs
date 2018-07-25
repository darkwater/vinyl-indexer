extern crate id3;
extern crate itertools;
extern crate metaflac;
extern crate rmp;

mod data;
mod error;

use data::*;
use error::Error;
use std::convert::Into;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[derive(Debug)]
struct Settings<'a> {
    root: &'a OsStr,
}

fn main() -> Result<(), Error> {
    let path = std::env::args_os().skip(1).next().unwrap();

    let settings = Settings {
        root: path.as_os_str(),
    };
    let (folders, errors) = walk_dir(&path, &settings);

    encode(folders, errors).unwrap();

    Ok(())
}

fn encode(folders: Vec<Folder>, errors: Vec<Error>) -> Result<(), Box<std::error::Error>> {
    use rmp::encode::*;
    let mut out = io::stdout();

    write_map_len(&mut out, 3)?;

    write_str(&mut out, "errors")?;
    write_uint(&mut out, errors.len() as u64)?;

    let fields = &[
        "name",         "format", "title", "artist",      "album",
        "album_artist", "year",   "disc",  "total_discs", "track",
        "total_tracks"
    ];

    // write_str(&mut out, "fields")?;
    // write_array_len(&mut out, fields.len() as u32)?;
    // for field in fields {
    //     write_str(&mut out, field)?;
    // }
    write_str(&mut out, "version")?;
    write_uint(&mut out, 1)?;

    write_str(&mut out, "folders")?;
    write_array_len(&mut out, folders.len() as u32)?;
    for folder in folders {
        write_map_len(&mut out, 2)?;

        write_str(&mut out, "path")?;
        write_bin(&mut out, folder.path.as_bytes())?;

        write_str(&mut out, "files")?;
        write_array_len(&mut out, folder.files.len() as u32)?;
        for file in folder.files {
            write_array_len(&mut out, fields.len() as u32)?;
            write_bin(&mut out, file.name.as_bytes())?;
            write_str(&mut out, file.format.to_str())?;
            write_str(&mut out, &file.title)?;
            write_str(&mut out, &file.artist)?;
            write_str(&mut out, &file.album)?;
            write_str(&mut out, &file.album_artist)?;
            write_sint(&mut out, file.year)?;
            write_sint(&mut out, file.disc)?;
            write_sint(&mut out, file.total_discs)?;
            write_sint(&mut out, file.track)?;
            write_sint(&mut out, file.total_tracks)?;
        }
    }

    Ok(())
}

fn walk_dir<P: AsRef<Path>>(path: P, settings: &Settings) -> (Vec<Folder>, Vec<Error>) {
    let mut folders = vec![];
    let mut errors  = vec![];

    let mut folder = Folder {
        // TODO: test symlinks leading out of the root
        path: path.as_ref().strip_prefix(settings.root).unwrap().as_os_str().to_os_string(),
        files: vec![],
    };

    let entries = fs::read_dir(path);
    if let Err(e) = entries {
        errors.push(e.into());
        return (folders, errors);
    }

    for entry in entries.unwrap() {
        let pt = entry.and_then(|f| f.file_type().map(|ftype| (f.path(), ftype)));
        if let Err(e) = pt {
            errors.push(e.into());
            continue;
        }
        let (path, ftype) = pt.unwrap();

        if ftype.is_dir() {
            let (mut ifolders, mut ierrors) = walk_dir(path, &settings);
            folders.append(&mut ifolders);
            errors.append(&mut ierrors);
            continue;
        }

        if ftype.is_file() {
            if let Ok(Some(file)) = read_file(path) {
                folder.files.push(file);
            }
        }
    }

    if folder.files.len() > 0 {
        folders.push(folder);
    }

    (folders, errors)
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<Option<File>, Error> {
    let path = path.as_ref();
    match path.extension().and_then(OsStr::to_str) {
        Some("flac") => {
            let meta = metaflac::Tag::read_from_path(path).map_err(error::failed_flac(path))?;
            let meta = meta.vorbis_comments().ok_or_else(error::missing("metadata", path))?;

            Ok(Some(File {
                name:         path.file_name().unwrap().to_os_string(),
                format:       Format::FLAC,
                title:        meta.get("TITLE")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.into())
                                  .cloned().unwrap_or("".to_string()),
                artist:       meta.get("ARTIST")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.into())
                                  .cloned().unwrap_or("".to_string()),
                album:        meta.get("ALBUM")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.into())
                                  .cloned().unwrap_or("".to_string()),
                album_artist: meta.get("ALBUMARTIST")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.into())
                                  .cloned().unwrap_or("".to_string()),
                year:         meta.get("YEAR")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.parse().ok())
                                  .unwrap_or(0),
                disc:         meta.get("DISCNUMBER")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.parse().ok())
                                  .unwrap_or(0),
                total_discs:  meta.get("TOTALDISCS")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.parse().ok())
                                  .unwrap_or(0),
                track:        meta.get("TRACKNUMBER")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.parse().ok())
                                  .unwrap_or(0),
                total_tracks: meta.get("TOTALTRACKS")
                                  .and_then(|n| n.first())
                                  .and_then(|n| n.parse().ok())
                                  .unwrap_or(0),
            }))
        },
        Some("mp3") => {
            let meta = id3::Tag::read_from_path(path).map_err(error::failed_id3(path))?;
            Ok(Some(File {
                name:         path.file_name().unwrap().to_os_string(),
                format:       Format::MP3,
                title:        meta.title().unwrap_or("").into(),
                artist:       meta.artist().unwrap_or("").into(),
                album:        meta.album().unwrap_or("").into(),
                album_artist: meta.album_artist().unwrap_or("").into(),
                year:         meta.year().map(|n| n as i64).unwrap_or(0),
                disc:         meta.disc().map(|n| n as i64).unwrap_or(0),
                total_discs:  meta.total_discs().map(|n| n as i64).unwrap_or(0),
                track:        meta.track().map(|n| n as i64).unwrap_or(0),
                total_tracks: meta.total_tracks().map(|n| n as i64).unwrap_or(0),
            }))
        },
        _ => Ok(None)
    }
}
