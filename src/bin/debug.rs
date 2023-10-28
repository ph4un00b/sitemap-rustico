use std::io::Write;
use std::process::Stdio;
use std::{fs::File, process::Command};
/*
 * usage:
 * cargo run --bin debug
 */

// todo: https://www.wikiwand.com/en/Sitemaps#Additional_sitemap_types
fn main() {
    let cmd = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg("--name-only")
        .arg("HEAD")
        .arg("--full-name")
        .arg("src")
        .current_dir("book")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Err on git");
    //? xargs -I {} -- git log -1 --format="%ad - {}" {}
    let cmd = Command::new("xargs")
        .arg("-I")
        .arg("{}")
        .arg("--")
        .arg("git")
        .arg("log")
        .arg("-1")
        //todo maybe format as iso
        .arg("--format=%ad - book/{}")
        .arg("{}")
        .stdin(cmd.stdout.expect("Failed to open stdout"))
        .current_dir("book")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Err on xargs");
    // * just *.md
    //? grep '\.md$'
    let cmd = Command::new("grep")
        .arg(r"\.md$")
        .stdin(cmd.stdout.expect("Failed to open stdout"))
        .current_dir("book")
        .output()
        .expect("Err on git");
    // let cmd = Command::new("ls")
    //     .arg("-1")
    //     .current_dir("book")
    //     .output()
    //     .expect("Err on git");
    let generated_file = "generator-000-book.txt";
    log_file(cmd, generated_file);

    let cmd = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg("--name-only")
        .arg("HEAD")
        .arg("--full-name")
        .arg("articles")
        .current_dir("blog")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Err on git");
    //? xargs -I {} -- git log -1 --format="%ad - {}" {}
    let cmd = Command::new("xargs")
        .arg("-I")
        .arg("{}")
        .arg("--")
        .arg("git")
        .arg("log")
        .arg("-1")
        //todo maybe format as iso
        .arg("--format=%ad - blog/{}")
        .arg("{}")
        .stdin(cmd.stdout.expect("Failed to open stdout"))
        .current_dir("blog")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Err on xargs");
    // * just *.md
    //? grep '\.md$'
    let cmd = Command::new("grep")
        .arg(r"\.md$")
        .stdin(cmd.stdout.expect("Failed to open stdout"))
        .current_dir("book")
        .output()
        .expect("Err on git");
    let generated_file = "generator-000-articles.txt";
    log_file(cmd, generated_file);

    // // * search blog/*.md files
    // // ? windows: busybox.exe find blog/articles blog/esta_semana_en_rust -type f
    // // ? for lowering deps we prefer two ls commands❗

    // let cmd = Command::new("ls")
    //     .arg("-1")
    //     .arg("blog/articles/*.md")
    //     .output()
    //     .expect("Err on ls");

    // let generated_file = "generator-001-blog-articles.txt";
    // log_file(cmd, generated_file);

    let cmd = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg("--name-only")
        .arg("HEAD")
        .arg("--full-name")
        .arg("esta_semana_en_rust")
        .current_dir("blog")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Err on git");
    //? xargs -I {} -- git log -1 --format="%ad - {}" {}
    let cmd = Command::new("xargs")
        .arg("-I")
        .arg("{}")
        .arg("--")
        .arg("git")
        .arg("log")
        .arg("-1")
        //todo maybe format as iso
        .arg("--format=%ad - blog/{}")
        .arg("{}")
        .stdin(cmd.stdout.expect("Failed to open stdout"))
        .current_dir("blog")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Err on xargs");
    // * just *.md
    //? grep '\.md$'
    let cmd = Command::new("grep")
        .arg(r"\.md$")
        .stdin(cmd.stdout.expect("Failed to open stdout"))
        .current_dir("book")
        .output()
        .expect("Err on git");
    let generated_file = "generator-000-week.txt";
    log_file(cmd, generated_file);

    // let cmd = Command::new("ls")
    //     .arg("-1")
    //     .arg("blog/esta_semana_en_rust/*.md")
    //     .output()
    //     .expect("Err on ls");

    // let generated_file = "generator-001-blog-week.txt";
    // log_file(cmd, generated_file);
    let cmd = Command::new("git")
        .arg("ls-tree")
        .arg("-r")
        .arg("--name-only")
        .arg("HEAD")
        .arg("--full-name")
        .arg("src/pages")
        .current_dir("home")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Err on git");
    //? xargs -I {} -- git log -1 --format="%ad - {}" {}
    let cmd = Command::new("xargs")
        .arg("-I")
        .arg("{}")
        .arg("--")
        .arg("git")
        .arg("log")
        .arg("-1")
        //todo maybe format as iso
        .arg("--format=%ad - home/{}")
        .arg("{}")
        .stdin(cmd.stdout.expect("Failed to open stdout"))
        .current_dir("home")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Err on xargs");
    // * just *.md
    //? grep '\.md$'
    let cmd = Command::new("grep")
        .arg(r"\.rs$")
        .stdin(cmd.stdout.expect("Failed to open stdout"))
        .current_dir("home")
        .output()
        .expect("Err on git");
    let generated_file = "generator-000-home.txt";
    log_file(cmd, generated_file);

    // let cmd = Command::new("ls")
    //     .arg("-1")
    //     .arg("home/src/pages/*.rs")
    //     .output()
    //     .expect("Err on ls");

    // let generated_file = "generator-001-home.txt";
    // log_file(cmd, generated_file);
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
        .arg("generator-*.txt")
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
    // * remove extension
    // ? awk -F "." '{print $1}' .\generator-002-grep.txt
    let cmd = Command::new("awk")
        // .args(&["-F", ".", "-f", "awk.txt", generated_file])
        .args(["-F", ".", "{print $1}", generated_file])
        .output()
        .expect("Err on awk");

    let generated_file = "generator-003-final.txt";
    log_file(cmd, generated_file);

    // let ignore = ["SUMMARY", "title-page", "mod" /* mod.rs */];

    // create_sitemap(generated_file);
}

fn log_file(cmd: std::process::Output, file_name: &str) {
    let out = String::from_utf8_lossy(&cmd.stdout);

    println!("Stdout: {}", out);
    println!("Stderr: {}", String::from_utf8_lossy(&cmd.stderr));

    let mut file = File::create(file_name).expect("Failed to create file");
    write!(file, "{}", out).expect("Failed to write to file");
}
