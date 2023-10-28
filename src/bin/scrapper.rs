use std::io::Write;
use std::{fs::File, process::Command};

/*
 * based on
 * @link: https://www.lostsaloon.com/technology/how-to-create-an-xml-sitemap-using-wget-and-shell-script/
 *
 * usage:
 * cargo run --
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

    let _cmd = Command::new(custom_ops.get("--wget-path").unwrap())
        .arg("--spider")
        .arg("--recursive")
        .arg("--no-verbose")
        .arg("--https-only")
        .arg("--no-clobber")
        //? .arg("--timestamping")
        //? .arg("--delete-after")
        //? .arg("--show-progress")
        //? .arg("--debug")
        //? .arg("--limit-rate=RATE")
        .arg("--output-file=debug-wget.txt")
        .arg(custom_ops.get("--root-site").unwrap())
        .output()
        .expect("- Failed to execute wget command\n- Did you install wget on windows?\n- Try: https://eternallybored.org/misc/wget\n");

    // println!("Status: {:?}", wget_out);
    // println!("Stdout: {}", String::from_utf8_lossy(&wget_out.stdout));
    // println!("Stderr: {}", String::from_utf8_lossy(&wget_out.stderr));
    //? grep -i URL .\sitemap.txt
    let cmd = Command::new("grep")
        .arg("-i")
        .arg("URL")
        .arg("debug-wget.txt")
        .output()
        .expect("Failed to execute command");

    //? Get the output as a string
    let out = String::from_utf8_lossy(&cmd.stdout);
    let mut file = File::create("debug-grep.txt").expect("Failed to create file");
    write!(file, "{}", out).expect("Failed to write to file");

    //? awk -F 'URL:' '{print $2}'
    let cmd = Command::new("awk")
        .args(["-F", "URL:", "{print $2}", "./debug-grep.txt"])
        .output()
        .expect("Failed to execute command");

    let out = String::from_utf8_lossy(&cmd.stdout);
    let mut file = File::create("debug-awk-1-column.txt").expect("Failed to create file");
    write!(file, "{}", out).expect("Failed to write to file");
    // println!("Status: {:?}", awk);
    // println!("Stdout: {}", String::from_utf8_lossy(&awk.stdout));
    // println!("Stderr: {}", String::from_utf8_lossy(&awk.stderr));

    //? trim spaces: awk '{$1=$1};1'
    let cmd = Command::new("awk")
        .args(["{$1=$1};1", "./debug-awk-1-column.txt"])
        .output()
        .expect("Failed to execute command");

    let out = String::from_utf8_lossy(&cmd.stdout);
    let mut file = File::create("debug-awk-2-trim.txt").expect("Failed to create file");
    write!(file, "{}", out).expect("Failed to write to file");
    // println!("Status: {:?}", awk);
    // println!("Stdout: {}", String::from_utf8_lossy(&awk.stdout));
    // println!("Stderr: {}", String::from_utf8_lossy(&awk.stderr));
    //? urls: awk '{print $1}'
    let cmd = Command::new("awk")
        .args(["{print $1}", "./debug-awk-2-trim.txt"])
        .output()
        .expect("Failed to execute command");

    let out = String::from_utf8_lossy(&cmd.stdout);
    let mut file = File::create("debug-awk-3-urls.txt").expect("Failed to create file");
    write!(file, "{}", out).expect("Failed to write to file");
    // println!("Status: {:?}", awk);
    // println!("Stdout: {}", String::from_utf8_lossy(&awk.stdout));
    // println!("Stderr: {}", String::from_utf8_lossy(&awk.stderr));

    let content =
        std::fs::read_to_string("debug-awk-3-urls.txt").expect("Failed to open to urls.txt file");

    use serde_derive::Deserialize;
    use serde_derive::Serialize;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[serde(rename = "urlset")]
    struct UrlSet {
        #[serde(rename = "@xmlns")]
        xmlns: String,
        url: Vec<Url>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Freq {
        //? {'always', 'hourly', 'daily', 'weekly', 'monthly', 'yearly', 'never'}.
        #[serde(rename = "always")]
        Always,
        #[serde(rename = "hourly")]
        Hourly,
        #[serde(rename = "daily")]
        Daily,
        #[serde(rename = "weekly")]
        Weekly,
        #[serde(rename = "monthly")]
        Monthly,
        #[serde(rename = "yearly")]
        Yearly,
        #[serde(rename = "never")]
        Never,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Changefreq {
        #[serde(rename = "$text")]
        field: Freq,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Url {
        #[serde(rename = "loc")]
        url: String,
        #[serde(rename = "lastmod")]
        updated_at: Option<String>,
        #[serde(rename = "changefreq")]
        freq: Option<Changefreq>,
        priority: Option<String>,
    }

    let items = content
        .lines()
        // .take(2)
        .map(|url| Url {
            url: url.to_owned(),
            updated_at: Some(iso_8601(&std::time::SystemTime::now())),
            freq: Some(Changefreq {
                field: Freq::Yearly,
            }),
            priority: Some("1.0".to_string()),
        })
        .collect::<Vec<Url>>();
    /*
     * The Sitemap XML protocol is also extended to provide
     * a way of listing multiple Sitemaps in a 'Sitemap index' file.
     * The maximum Sitemap size of 50 MiB or 50,000 URLs means this is necessary for large sites.
     *
     * @see https://www.wikiwand.com/en/Sitemaps#File_format
     */
    let xml = quick_xml::se::to_string(&UrlSet {
        url: items,
        xmlns: "http://www.sitemaps.org/schemas/sitemap/0.9".to_string(),
    })
    .unwrap();

    let mut file = File::create("sitemap.xml").expect("Failed to create file");
    write!(file, "{}", xml).expect("Failed to write sitemap.xml file");

    let cmd = Command::new("xmllint")
        .arg("--schema")
        .arg("schema.xsd")
        .arg("sitemap.xml")
        .output()
        .expect("- Failed to execute xmllint command\n- Did you install xmllint on windows?\n- Try: scoop install libxml2\n");

    // println!("Status: {:?}", cmd);
    // println!("Stdout: {}", String::from_utf8_lossy(&cmd.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&cmd.stderr));
}

fn iso_8601(system_time: &std::time::SystemTime) -> String {
    use chrono::prelude::{DateTime, Utc};
    // * @see https://www.w3.org/TR/NOTE-datetime
    let datetime: DateTime<Utc> = (*system_time).into();
    //? formats like "2001-07-08T00:34:60.026490+09:30"
    format!("{}", datetime.format("%+"))
}
