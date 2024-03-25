use std::{io::Write, process::Stdio};

use crate::hf::ToDownload;

pub struct Aria2Options {
    pub destination: String,
    pub par_files: u32,
    pub streams: u32,
}

pub fn download_model(options: &Aria2Options, files: Vec<ToDownload>) {
    println!("Starting download...");
    let spec = create_aria2_spec(files);

    let mut aria2c = std::process::Command::new("aria2c")
        .arg("--input-file=-")
        .arg(format!("--dir={}", options.destination))
        .arg(format!("--max-concurrent-downloads={}", options.par_files))
        .arg(format!("--max-connection-per-server={}", options.streams))
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to start aria2c");
    aria2c.stdin
        .take()
        .unwrap()
        .write_all(spec.as_bytes())
        .expect("Failed to write to aria2c");

    match aria2c.wait() {
        Ok(status) => {
            if !status.success() {
                panic!("Failed to download model");
            }
        }
        Err(err) => {
            panic!("Failed to wait for aria2c: {}", err);
        }
    }
}

fn create_aria2_spec(files: Vec<ToDownload>) -> String {
    let mut spec = String::new();
    for resource in files {
        spec.push_str(&format!("{}\n", resource.url));
        spec.push_str(&format!("\tout={}\n", resource.path));
        spec.push_str(&format!("\tcontinue=true\n"));
    }
    spec
}