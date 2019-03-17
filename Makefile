# -*- coding: utf-8-unix -*-

DESTDIR  =
PREFIX   = /usr/local
BINDIR   = $(DESTDIR)$(PREFIX)/bin
THUMBDIR = $(DESTDIR)$(PREFIX)/share/thumbnailers

mpv-thumbnailer: mpv-thumbnailer.rs
	rustc mpv-thumbnailer.rs

clean:
	rm -f mpv-thumbnailer

install:
	test -s mpv-thumbnailer
	mkdir -p $(BINDIR)
	cp mpv-thumbnailer $(BINDIR)
	chmod +x $(BINDIR)/mpv-thumbnailer
	mkdir -p $(THUMBDIR)
	cp mpv-thumbnailer.thumbnailer $(THUMBDIR)

.PHONY: clean install
