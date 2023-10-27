use std::io::Write;
use std::{fs::File, process::Command};

fn main() {
    let cmd = Command::new("cat")
        .arg("gen-*.txt")
        .output()
        .expect("Err on merging");

    let generated_file = "generator-002-merged.txt";
    log_file(cmd, generated_file);
}

fn log_file(cmd: std::process::Output, file_name: &str) {
    let out = String::from_utf8_lossy(&cmd.stdout);

    println!("Stdout: {}", out);
    println!("Stderr: {}", String::from_utf8_lossy(&cmd.stderr));

    let mut file = File::create(file_name).expect("Failed to create file");
    write!(file, "{}", out).expect("Failed to write to file");
}
