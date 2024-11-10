use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::{env, io, thread};
use std::path::Path;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Result};

#[derive(Debug)]
struct MyFile {
    path: String,
    name: String,
    //content: String,
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
        Sha256::update(&mut sha, &buffer);
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
        // }
    }
    return Ok(vec);
}
fn delete_file(dup_file: MyFile) {
    fs::remove_file(dup_file.path).unwrap();
}

fn main() -> io::Result<()> {
    let mut dry_run = true;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        return Err(io::Error::from(io::ErrorKind::InvalidInput));
    }
    if args.len() ==3 && &args[2] == "--real-run" {
        dry_run =false;
    }
    let path = &args[1];

    if Path::new(path).is_dir() {
        let mut map = HashMap::<String, MyFile>::new();  // Change the key type to String
        let files = work(&path)?;
        let mut count_dups =0;
        let mut count_origs =0;
        for file in files {
            match map.get(&file.hash) {
                None => {
                    map.insert(file.hash.clone(), file);  // Insert file.hash as a String
                    count_origs +=1;
                }
                Some(_) => {
                    println!("{} Delete", &file.name);
                    count_dups +=1;
                    if !dry_run {
                        delete_file(file);
                    }
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
