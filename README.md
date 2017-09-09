mpv-thumbnailer
===============

A simple video thumbnailer that uses [mpv](https://mpv.io/), primarily
intended to be used by a file browser. If you use mpv as your video
player and are happy with its playback, you'll likely be happy with
mpv-thumbnailer's thumbnails. File format support should be good and
aspect ratio errors rare.

mpv-thumbnailer includes and installs a thumbnailer definition file that
works with GNOME's Nautilus. Support for other platforms or file
browsers should be easy to add.

```
Usage: mpv-thumbnailer -i VIDEO -o THUMBNAIL [-s SIZE]

VIDEO is the input file to generate a thumbnail from and THUMBNAIL is
the output image file to write. SIZE is optional and defaults to 128.
```

mpv-thumbnailer requires [mpv](https://mpv.io/) and
[ImageMagick](https://www.imagemagick.org/).
