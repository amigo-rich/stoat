use crate::error::Error;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub fn build_path_hash_vec_from_path(path: PathBuf) -> Result<Vec<(String, PathBuf)>, Error> {
    if !path.is_dir() {
        return Err(Error::NotADir(path.to_path_buf()));
    }

    let (sender, receiver) = channel();
    thread::spawn(move || {
        visit_dirs(&path, &sender).unwrap();
    });

    let mut path_hash: Vec<(String, PathBuf)> = Vec::new();
    for pb in receiver {
        let content = fs::read(&pb);
        if content.is_ok() {
            let hash = blake3::hash(&content.unwrap());
            path_hash.push((hash.to_string(), pb));
        }
    }

    Ok(path_hash)
}

// borrowed/adapted from the rust docs
fn visit_dirs(dir: &Path, sender: &Sender<PathBuf>) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let pb = entry.path();
            if pb.is_dir() {
                visit_dirs(&pb, sender)?;
            } else {
                sender.send(pb)?;
            }
        }
    }
    Ok(())
}
