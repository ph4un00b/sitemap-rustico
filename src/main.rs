use std::io::Write;
use std::{fs::File, process::Command};

/*
 * based on
 * @link: https://www.lostsaloon.com/technology/how-to-create-an-xml-sitemap-using-wget-and-shell-script/
 *
 * usage:
 * cargo run --bin sitemap-rustico
 * or
 * cargo run -- --wget-path C:\path\to\wget.exe
 */
// todo: https://www.wikiwand.com/en/Sitemaps#Additional_sitemap_types
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut custom_ops = std::collections::HashMap::new();

    custom_ops.insert("--wget-path", "wget");
    custom_ops.insert("--root-site", "https://rustlanges.github.io/blog/");

    for flags in args.chunks(2) {
        match (flags[0].as_str(), flags[1].as_str()) {
            ("--wget-path", path) => {
                println!("path {path}");
                custom_ops.insert("--wget-path", path);
            }
            _ => {
                eprintln!("Invalid input");
                println!("Try: --wget-path C:\\your\\path\\wget.exe");
                std::process::exit(0)
            }
        }
    }

    let original = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let group_size = 3;

    for x in original.chunks(group_size) {
        println!("Resultado: {:?}", x);
    }
}
