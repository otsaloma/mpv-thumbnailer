// -*- coding: utf-8-unix -*-

use std::env;
use std::process::Command;
use std::process::ExitStatus;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: mpv-thumbnailer VIDEO THUMBNAIL SIZE\n");
        println!("VIDEO is the input video file to generate a thumbnail from.");
        println!("THUMBNAIL is the output image file to write.");
        println!("SIZE is the pixel width/height of the thumbnail image.");
        exit(1);
    }
    // Seeking might fail with some semi-broken files.
    // Fall back to thumbnailing from the start.
    for start in vec!["25%", "0"] {
        let status = thumbnail(&args[1], &args[2], &args[3], &start.to_string());
        status.success() && exit(0);
    }
    exit(1);
}

fn thumbnail(input: &String, output: &String, size: &String, start: &String) -> ExitStatus {
    // XXX: We can't seem to set scaling by the maximum dimension,
    // so for portrait videos we get a height over the requested size.
    return Command::new("mpv")
        .arg("--really-quiet")
        .arg("--no-config")
        .arg("--aid=no")
        .arg("--sid=no")
        .arg(format!("--vf=scale={}:-2", size))
        .arg(format!("--start={}", start))
        .arg("--frames=1")
        .arg(format!("--o={}", output))
        .arg(format!("{}", input))
        .status()
        .expect("failed to execute mpv process");

}
