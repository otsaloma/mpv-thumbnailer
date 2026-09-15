// -*- coding: utf-8-unix -*-

use std::env;
use std::fs;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::process;
use std::process::Command;
use std::process::ExitStatus;

fn main() {
    let args: Vec<String> = env::args().collect();
    let [_, input, output, size, ..] = &args[..] else {
        println!("Usage: mpv-thumbnailer VIDEO THUMBNAIL SIZE\n");
        println!("VIDEO is the input video file to generate a thumbnail from.");
        println!("THUMBNAIL is the output image file to write.");
        println!("SIZE is the pixel width/height of the thumbnail image.");
        process::exit(1);
    };
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let id = hasher.finish();
    // Thumbnail multiple times and copy the largest file as output,
    // assuming it's the least likely to be a boring all-black frame.
    // Note that seeking might fail with some semi-broken files, so
    // always fall back to thumbnailing from the start.
    let mut thumbnails = Vec::new();
    for start in ["25%", "20%", "15%", "0"] {
        let path = env::temp_dir().join(
            format!("mpv-thumbnailer-{}-{}.png", id, start.replace("%", "")));
        let status = thumbnail(input, &path, size, start);
        if !status.success() || !path.exists() { continue; }
        let fsize = path.metadata().expect("failed to get metadata").len();
        thumbnails.push((path, fsize));
        if thumbnails.len() >= 3 { break; }
    }
    let Some((largest, _)) = thumbnails.iter().max_by_key(|(_, fsize)| fsize) else {
        process::exit(1);
    };
    fs::copy(largest, output).expect("failed to copy file");
    for (path, fsize) in &thumbnails {
        println!("{:?}: {:?}", path, fsize);
        fs::remove_file(path).expect("failed to remove file");
    }
}

fn thumbnail(input: &str, output: &Path, size: &str, start: &str) -> ExitStatus {
    // XXX: We can't seem to set scaling by the maximum dimension,
    // so for portrait videos we get a height over the requested size.
    Command::new("mpv")
        .arg("--really-quiet")
        .arg("--no-config")
        .arg("--aid=no")
        .arg("--sid=no")
        .arg(format!("--vf=scale={}:{}/dar", size, size))
        .arg(format!("--start={}", start))
        .arg("--frames=1")
        .arg(format!("--o={}", output.display()))
        .arg(input)
        .status()
        .expect("failed to execute mpv process")
}
