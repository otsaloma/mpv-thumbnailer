mpv-thumbnailer
===============

A simple video thumbnailer that uses [mpv](https://mpv.io/), primarily
intended to be used by a file browser. If you use mpv as your video
player and are happy with its playback, you'll likely be happy with
mpv-thumbnailer's thumbnails. File format support should be good and
aspect ratio errors rare.

mpv-thumbnailer generates multiple thumbnails per file and selects the
largest of those files, thus hopefully avoiding the common problem of
thumbnailing boring all-black frames. As a downside, this means that
mpv-thumbnailer is somewhat slow.

mpv-thumbnailer includes and installs a thumbnailer definition file that
works with GNOME's Nautilus. Support for other platforms or file
browsers should be easy to add.

```
Usage: mpv-thumbnailer VIDEO THUMBNAIL SIZE

VIDEO is the input video file to generate a thumbnail from.
THUMBNAIL is the output image file to write.
SIZE is the pixel width/height of the thumbnail image.
```

## Installation

To install, run

```bash
make
make PREFIX=... install
```

For the `make` step you need the Rust compiler `rustc`.

## Development Notes

Since version 3.26 GNOME runs thumbnailers sandboxed via Bubblewrap. Any
changes should be tested to adhere to those sandbox rules, which
currently seem to only be documented in the below bug report.

<https://bugzilla.gnome.org/show_bug.cgi?id=774497>
