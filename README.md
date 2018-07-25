`vinyl-indexer`
===============

`vinyl-indexer` crawls a folder for music files, and outputs a list containing track title, artist, etc. for each file.

Usage
-----

    vinyl-indexer music_root

Output format
-------------

`vinyl-indexer` outputs in [msgpack](https://msgpack.org/) by default.

```c
    map
    "errors": uint      // to be implemented
    "version": uint     // sequential version number of protocol
    "folders": array    // all folders found on any depth
        map             //   a single folder
        "path": binary  //     folder path from music_root
        "files": array  //     all files directly in this folder
            array       //       a single music file
              binary    //         first field
              string    //         second field
              string    //         third field
              ...       //         more fields
            ...         //       more files
        ...             //   more folders
```

### Fields

Values of `0` or `""` usually indicate missing metadata.

```c
    filename:     binary
    format:       string,
    title:        string,
    artist:       string,
    album:        string,
    album_artist: string,
    year:         sint,
    disc:         sint,
    total_discs:  sint,
    track:        sint,
    total_tracks: sint,
```
