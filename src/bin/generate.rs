use std::io::Write;
use std::{fs::File, process::Command};
/*
 * usage:
 * cargo run --bin generate
 */

// todo: https://www.wikiwand.com/en/Sitemaps#Additional_sitemap_types
fn main() {
    let cmd = Command::new("ls")
        .arg("-1")
        .arg("book/src/*md")
        .output()
        .expect("Err on ls");

    let generated_file = "generator-001-book.txt";
    log_file(cmd, generated_file);
    // * search blog/*.md files
    // ? windows: busybox.exe find blog/articles blog/esta_semana_en_rust -type f
    // ? for lowering deps we prefer two ls commands❗

    let cmd = Command::new("ls")
        .arg("-1")
        .arg("blog/articles/*.md")
        .output()
        .expect("Err on ls");

    let generated_file = "generator-001-blog-articles.txt";
    log_file(cmd, generated_file);

    let cmd = Command::new("ls")
        .arg("-1")
        .arg("blog/esta_semana_en_rust/*.md")
        .output()
        .expect("Err on ls");

    let generated_file = "generator-001-blog-week.txt";
    log_file(cmd, generated_file);

    let cmd = Command::new("ls")
        .arg("-1")
        .arg("home/src/pages/*.rs")
        .output()
        .expect("Err on ls");

    let generated_file = "generator-001-home.txt";
    log_file(cmd, generated_file);
    // * add blog tags
    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("tags")
        .output()
        .expect("Err on cargo");

    let generated_file = "generator-001-tags.txt";
    log_file(cmd, generated_file);
    // * merge files
    let cmd = Command::new("cat")
        .arg("generator-001-book.txt")
        .arg("generator-001-blog-articles.txt")
        .arg("generator-001-blog-week.txt")
        .arg("generator-001-home.txt")
        .arg("generator-001-tags.txt")
        .output()
        .expect("Err on merging");

    let generated_file = "generator-002-merged.txt";
    log_file(cmd, generated_file);
    // // * remove prefix
    // // ? cat .\generator-001-ls.txt | sed 's/.*\///'
    // // or
    // // ? grep -o '[^/]*$' .\generator-001-ls.txt
    // let cmd = Command::new("grep")
    //     .arg("-o")
    //     .arg("[^/]*$")
    //     .arg(generated_file)
    //     .output()
    //     .expect("Err on grep");

    // let generated_file = "generator-002-grep.txt";
    // log_file(cmd, generated_file);
    // * remove md extension
    // ? awk -F "." '{print $1}' .\generator-002-grep.txt
    let cmd = Command::new("awk")
        // .args(&["-F", ".", "-f", "awk.txt", generated_file])
        .args(&["-F", ".", "{print $1}", generated_file])
        .output()
        .expect("Err on awk");

    let generated_file = "generator-003-merged.txt";
    log_file(cmd, generated_file);

    let ignore = ["SUMMARY", "title-page", "mod" /* mod.rs */];
}

fn log_file(cmd: std::process::Output, file_name: &str) {
    let out = String::from_utf8_lossy(&cmd.stdout);

    println!("Stdout: {}", out);
    println!("Stderr: {}", String::from_utf8_lossy(&cmd.stderr));

    let mut file = File::create(file_name).expect("Failed to create file");
    write!(file, "{}", out).expect("Failed to write to file");
}
