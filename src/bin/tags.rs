use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;

fn main() {
    let articles_tags = tags("./blog/articles");
    // println!("{:?}", articles_tags);
    let mut tags = tags("./blog/esta_semana_en_rust");
    // println!("{:?}", tags.extend(articles_tags));
    tags.extend(articles_tags);
    //todo tags from dev-to, hash-node
    for tag in tags {
        println!("blog/tags/{tag}");
    }
}

fn tags(path: &str) -> HashSet<String> {
    let result: HashSet<String> = fs::read_dir(path)
        .expect("Failed to read /articles")
        .map(|path| match path {
            Ok(dir) => dir,
            Err(_) => panic!("some err!"),
        })
        .map(|entry| {
            let matter: Matter<YAML> = Matter::new();
            let raw = fs::read_to_string(entry.path()).expect("{entry:?} fail");
            let result = matter.parse(&raw);
            result
        })
        .map(|result| {
            #[derive(Deserialize, Debug)]
            struct FrontMatter {
                tags: Vec<String>,
            }
            let front_matter: FrontMatter = result.data.unwrap().deserialize().unwrap();
            front_matter.tags
        })
        // .take(1)
        .flatten()
        .map(|tag| tag.to_lowercase().replace(' ', "-"))
        .collect::<HashSet<String>>();
    result
}
