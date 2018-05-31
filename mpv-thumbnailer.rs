// -*- coding: utf-8-unix -*-

use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;
use std::process;
use std::process::Command;
use std::process::ExitStatus;

fn hash<T: Hash>(data: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    return hasher.finish();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: mpv-thumbnailer VIDEO THUMBNAIL SIZE\n");
        println!("VIDEO is the input video file to generate a thumbnail from.");
        println!("THUMBNAIL is the output image file to write.");
        println!("SIZE is the pixel width/height of the thumbnail image.");
        process::exit(1);
    }
    // Thumbnail multiple times and copy the largest file as output,
    // assuming it's the least likely to be a boring all-black frame.
    // Note that seeking might fail with some semi-broken files, so
    // always fall back to thumbnailing from the start.
    let mut fnames = Vec::new();
    let mut fsizes = Vec::new();
    for start in vec!["25%", "20%", "15%", "0"] {
        let mut path = env::temp_dir();
        path.push(format!("mpv-thumbnailer-{}-{}.png",
                          hash(&args[1]),
                          start.replace("%", "")));

        let fname = String::from(path.to_string_lossy());
        let status = thumbnail(&args[1], &fname, &args[3], &start.to_string());
        if !status.success() || !path.exists() { continue; }
        let metadata = path.metadata().expect("failed to get metadata");
        fnames.push(fname.clone());
        fsizes.push(metadata.len().clone());
        if fnames.len() >= 3 { break; }
    }
    if fnames.len() == 0 { process::exit(1); }
    let max_fsize = fsizes.iter().max().unwrap();
    let max_index = fsizes.iter().position(|&x| x == *max_fsize).unwrap();
    fs::copy(&fnames[max_index], &args[2]).expect("failed to copy file");
    for (i, fname) in fnames.iter().enumerate() {
        println!("{:?}: {:?}", fnames[i], fsizes[i]);
        fs::remove_file(fname).expect("failed to remove file");
    }
    process::exit(0);
}

fn thumbnail(input: &String, output: &String, size: &String, start: &String) -> ExitStatus {
    // XXX: We can't seem to set scaling by the maximum dimension,
    // so for portrait videos we get a height over the requested size.
    return Command::new("mpv")
        .arg("--really-quiet")
        .arg("--no-config")
        .arg("--aid=no")
        .arg("--sid=no")
        .arg(format!("--vf=scale={}:{}/dar", size, size))
        .arg(format!("--start={}", start))
        .arg("--frames=1")
        .arg(format!("--o={}", output))
        .arg(format!("{}", input))
        .status()
        .expect("failed to execute mpv process");

}
