use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::fs::{self, DirEntry};
use std::io::{Read, Result};
use std::path::Path;
use std::{env, io};

#[derive(Debug)]
struct MyFile {
    path: String,
    name: String,
    hash: String,
}
impl MyFile {
    fn new(entry: DirEntry) -> Result<MyFile> {
        let mut file = File::open(entry.path())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let mut file = MyFile {
            path: entry.path().to_str().unwrap().to_string(),
            name: entry.file_name().to_str().unwrap().to_string(),
            hash: "".to_string(),
        };
        let mut sha = Sha256::new();
        sha.update(&buffer);
        let res = Sha256::finalize(sha);
        let v = res.iter().map(|x| format!("{:02x}", x)).collect::<Vec<String>>().join("");
        file.hash = v;
        return Ok(file);
    }
}


fn work(_path: &str) -> Result<Vec<MyFile>> {
    let path = Path::new(_path);
    let mut vec: Vec<MyFile> = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            continue;
        }
        let file = MyFile::new(entry);
        vec.push(file?);
    }
    return Ok(vec);
}
fn delete_file(dup_file: &MyFile) {
    fs::remove_file(&dup_file.path).unwrap();
}

fn help() {
    println!("example usage:");
    println!("duplicate-remover <path> <options>");
    println!("options:");
    println!("--real-run : deletes your duplicate files");
}
fn main() -> io::Result<()> {
    let mut dry_run = true;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return Ok(());
    }
    if args.len() == 3 && &args[2] == "--real-run" {
        dry_run = false;
    }
    if &args[1] == "-h" {
        help();
        return Ok(());
    }
    let path = &args[1];

    if Path::new(path).is_dir() {
        let mut map = HashMap::<String, MyFile>::new();
        let files = work(&path)?;
        let mut count_dups = 0;
        let mut count_origs = 0;
        for file in files {
            match map.get(&file.hash) {
                None => {
                    map.insert(file.hash.clone(), file);
                    count_origs += 1;
                }
                Some(old) => {
                    if old.name.len() > file.name.len() {
                        println!("{} Delete", &old.name);
                        if !dry_run {
                            delete_file(&old);
                        }
                        map.insert(file.hash.clone(), file);
                    } else {
                        println!("{} Delete", &file.name);
                        if !dry_run {
                            delete_file(&file);
                        }
                    }
                    count_dups += 1;
                }
            }
        }
        println!("Total Dups: {}", count_dups);
        println!("Total Origs: {}", count_origs);
    } else {
        println!("The path provided is not a directory.");
    }
    return Ok(());
}
