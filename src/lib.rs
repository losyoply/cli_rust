use std::fs;
use std::fs::ReadDir;

pub fn run_ls(path: &str) -> ReadDir {
    let paths = fs::read_dir(path).unwrap();
    paths
}