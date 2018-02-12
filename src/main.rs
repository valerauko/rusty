use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::process::Command;

fn main() {
    let path = env::args().nth(1).expect("I'll need a file to open...");

    let mut m3u = match File::open(path) {
        Err(why) => panic!("Failed to open file: {}", why.description()),
        Ok(file) => file
    };

    let mut data = String::new();
    match m3u.read_to_string(&mut data) {
        Err(why) => panic!("Failed to read file: {}", why.description()),
        Ok(_) => ()
    }

    let target = &env::args().nth(2).expect("I'll need a folder to copy to...");
    let writable = Command::new("gio").arg("info").args(&["-a", "access::can-write"]).arg(target)
                                      .output().expect("Error checking target folder.");
    let result = String::from_utf8_lossy(&writable.stdout);
    if &result[result.len()-5..result.len()-1] != "TRUE" {
        panic!("Cannot write target folder: {}.", &result[result.len()-5..result.len()-1])
    }

    for line in data.lines() {
        if line.chars().next().unwrap() == '#' {
            continue;
        }
        copy_file(line, target);
    }
}

fn copy_file(file: &str, base: &String) {
    let path = Path::new(file);
    if !path.exists() {
        println!("File '{}' doesn't exist.", file);
        return;
    }

    let s = Command::new("gio").arg("copy").arg(path).arg(base).output().expect("Murr");
    if s.status.success() {
        let file = path.file_name().unwrap().to_str().unwrap();
        println!("File copied: {}", file);
    }
}
