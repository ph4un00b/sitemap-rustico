use std::collections::HashMap;
use std::io::Write;
use std::{fs::File, process::Command};
/*
 * usage:
 * 
 * windows: ./generate.bat
 * unix: ./generate.sh
 * 
 * @see https://www.sitemaps.org/protocol.html
 * @see https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap
 * @see https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap#addsitemap
 * @see https://www.wikiwand.com/en/Sitemaps#Additional_sitemap_types
 */
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

fn main() {

    let filename = "generated/dates_and_paths.txt";

    let ignore = [
        "book/src/SUMMARY",
        // "home/src/pages/communidad",   /* communidad.rs */
        // "home/src/pages/contributors", /* contributors.rs */
        "home/src/pages/mod",          /* mod.rs */
        /* dotnet*/
        "dotnet/src/es/SUMMARY",
        "dotnet/src/es/license",
    ];
    let content = std::fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to open '{filename}' file"));

    use std::collections::HashSet;
    let mut seen = HashSet::new();

    let items = content
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.splitn(2, ',').collect();
            if parts.len() != 2 {
                panic!("Expected each line to contain exactly one comma.");
            }
            (parts[0], parts[1])
        })
        .filter(|(_date, path)| !ignore.contains(path))
        .map(|(date, path)| {
            let cleaned_path = path.strip_suffix("/index").unwrap_or(path).to_string();
            (date, cleaned_path)
        })
        // filter duplicated paths
        .filter(|(_date, path)| seen.insert(path.clone()))
        .map(|(date, path)| {
            /*
             * structure samples:
             *
             * home: BASE_URL/colaboradores (dynamic)
             * blog: BASE_URL/blog/index.html
             * blog/weekly: BASE_URL/blog/articles/2023-10-18-this-week-in-rust.html
             * blog/article: BASE_URL/blog/articles/strings.html
             * blog/tag: BASE_URL/blog/tags/data-type.html
             * book: BASE_URL/rust-book-es/ch01-00-getting-started.html
             */

            /*
             * 🔥 beware: esto modifica todo los enlaces❗❗
             */

             let dominio = "rustlang-es.org";
             let mut subdominios = HashMap::new();
            subdominios.insert("home", "https://rustlang-es.org");
            subdominios.insert("blog", "https://blog.rustlang-es.org");
            subdominios.insert("book", "https://book.rustlang-es.org");
            subdominios.insert("dotnet", "https://dotnet-book.rustlang-es.org");

            let parts: Vec<&str> = path.split('/').collect();

            println!(">>>> {:?}", parts);
            
            let (namespace, url) = match (parts[0], parts[1]) {
                // todo: las páginas dinámicas sólo redirigen al /
                // ?path is home/src/pages/path
                ("home", "src") => {
                    let namespace = format!("{}/{}", parts[0], parts[1]);
                    let base_url = subdominios.get("home").unwrap();

                    if let Some(page) = parts.get(3) {
                        let string_url = format!("{base_url}/{}", page);
                        (namespace, string_url)
                    } else {
                        let string_url = format!("{base_url}/");
                        (namespace, string_url)
                    }
                },

                ("blog", "articles") => {
                    let namespace = format!("{}/{}", parts[0], parts[1]);
                    let base_url = subdominios.get("blog").unwrap();
                    let string_url = format!("{base_url}/articles/{}", parts[2]);

                    (namespace, string_url)
                },

                ("blog", "esta_semana_en_rust") => {
                    let namespace = format!("{}/{}", parts[0], parts[1]);
                    let base_url = subdominios.get("blog").unwrap();
                    let string_url = format!("{base_url}/articles/{}", parts[2]);

                    (namespace, string_url)
                }

                ("blog", "tags") => {
                    let namespace = format!("{}/{}", parts[0], parts[1]);
                    let base_url = subdominios.get("blog").unwrap();
                    let string_url = format!("{base_url}/blog/tags/{}", parts[2]);

                    (namespace, string_url)
                }

                ("book", "src") => {
                    let namespace = format!("{}/{}", parts[0], parts[1]);
                    let base_url = subdominios.get("book").unwrap();
                    let string_url = format!("{base_url}/{}", parts[2]);

                    (namespace, string_url)
                }

                ("dotnet", "src") => {
                    let namespace = format!("{}/{}", parts[0], parts[1]);
                    let base_url = subdominios.get("dotnet").unwrap();

                    if let Some(page) = parts.get(5) {
                        let string_url = format!("{base_url}/{}/{}/{}", parts[3], parts[4], page);
                        (namespace, string_url)
                    } else if let Some(page) = parts.get(4) {
                        let string_url = format!("{base_url}/{}/{}", parts[3], page);
                        (namespace, string_url)
                    } else {
                        let string_url = format!("{base_url}/{}", parts[3]);
                        (namespace, string_url)
                    }
                }
                _ => panic!("invalid namespace❗: {parts:?}"),
            };

            println!("{:?}\n", url);
            (date, namespace, url)
        })
        .map(|(date, namespace, path)| Url {
            url: path.to_owned(),
            updated_at: if date.is_empty() {
                // todo: if this is the case we need to find another way to fetch the real date
                Some(iso_8601(&std::time::SystemTime::now()))
            } else {
                Some(date.to_string())
            },
            freq: match namespace.as_str() {
                "home/src" => Some(Changefreq {
                    field: Freq::Monthly,
                }),
                "blog/articles" => Some(Changefreq {
                    field: Freq::Monthly,
                }),
                "blog/esta_semana_en_rust" => Some(Changefreq {
                    field: Freq::Monthly,
                }),
                "blog/tags" => Some(Changefreq {
                    field: Freq::Monthly,
                }),
                "book/src" => Some(Changefreq {
                    field: Freq::Yearly,
                }),
                "dotnet/src" => Some(Changefreq {
                    field: Freq::Yearly,
                }),
                _ => panic!("invalid namespace❗: {namespace:?}"),
            },
            priority: match namespace.as_str() {
                "home/src" => Some("0.75".to_string()),
                "blog/articles" => Some("0.75".to_string()),
                "blog/esta_semana_en_rust" => Some("0.75".to_string()),
                "blog/tags" => Some("0.75".to_string()),
                "book/src" => Some("1.0".to_string()),
                "dotnet/src" => Some("1.0".to_string()),
                _ => panic!("invalid namespace❗: {namespace:?}"),
            },
            // priority: Some("1.0".to_string()),
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

    validate_sitemap();
    println!("👀 total links:");
    println!("{}", seen.len());
}

fn validate_sitemap() {
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
