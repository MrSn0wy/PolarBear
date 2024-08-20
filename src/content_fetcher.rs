use std::fs;
use std::path::{Path, PathBuf};

use crate::eprintln_red;
use crate::needed::Polar;

pub(crate) fn path_retriever(uri: String) -> Polar<PathBuf> {
    // get root path of website
    let local_path = Path::new(".");
    let resolved_local_path = match local_path.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            eprintln_red!("Error while getting root path of website!");
            return Polar::Silly(500); // Return 500
        }
    };

    // get fully resolved path of request
    let path_string = format!(".{}", uri);
    let path = Path::new(&path_string);

    let mut resolved_path = match path.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            eprintln_red!("User gave invalid path! || {:?}", path);
            return Polar::Silly(404); // Return 404
        }
    };

    if resolved_path.is_dir() {
        resolved_path.push("index.html")
    }

    // check if the request is in bounds of the website
    if !resolved_path.starts_with(resolved_local_path) {
        eprintln_red!("User tried to access out of bounds path! || {:?}", resolved_path);
        Polar::Silly(404) // should prob make it return code 403, however it might be a security issue, so 404 will suffice
    } else {
        Polar::Some(resolved_path)
    }
}

pub(crate) fn file_retriever(path: PathBuf) -> Polar<Vec<u8>> {
    if path.is_file() {
        match fs::read(&path) {
            Ok(file) => Polar::Some(file),
            Err(err) => {
                eprintln_red!("Error while trying to request {:?} || Msg: {}", path, err);
                Polar::Silly(500) // internal server error ig
            }
        }
    } else if path.is_dir() {
        Polar::Silly(500) // shouldn't happen since we do a check while getting the path, but if it does, oopsie woopsie!
    } else {
        Polar::Silly(404) // not found!
    }
}