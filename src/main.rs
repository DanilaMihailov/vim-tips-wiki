use select::document::Document;
use select::document::Find;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct WikiEntry<'a> {
    n: u32,
    url: String,
    title: String,
    categories: Vec<String>,
    author: Option<String>,
    created: u32,
    complexity: String,
    version: String,
    nodes: Vec<Node<'a>>,
}

impl<'a> WikiEntry<'a> {
    fn file_name(&self) -> String {
        self.prefix(".txt")
    }

    fn short_prefix(&self, txt: &str) -> String {
        format!("vwt-{}{}", self.n, txt)
    }

    fn prefix(&self, txt: &str) -> String {
        format!("vim-wiki-tips-{}{}", &self.title.to_lowercase(), txt)
    }

    fn to_vim_help(&self) -> String {
        let fname = self.file_name();
        let mut result = String::new();

        // add first row
        result.push_str(&format!("*{}*   {}\n\n", self.file_name(), self.title));
        result.push_str(&format!("created: {}\n", self.created));
        result.push_str(&format!("complexity: {}\n", self.complexity));
        if let Some(author) = &self.author {
            result.push_str(&format!("author: {}\n", author));
        }
        result.push_str(&format!("version: {}\n\n", self.version));

        for node in &self.nodes {
            match node.name() {
                Some("p") => {
                    let text = node.text();
                    let mut new_text = String::with_capacity(text.len());
                    let mut col = 1;

                    for word in text.trim().split_whitespace() {
                        col += word.chars().count() + 1;
                        if col > 77 {
                            new_text.push('\n');
                            col = 1;
                        };

                        new_text.push_str(word);
                        new_text.push(' ');
                    }

                    result.push_str(&format!("{}\n\n", new_text.trim()))
                }
                Some("pre") => result.push_str(&format!(
                    ">\n    {}\n<\n",
                    node.text()
                        .trim()
                        .split('\n')
                        .collect::<Vec<&str>>()
                        .as_slice()
                        .join("\n    ")
                )),
                Some("h2") | Some("h3") => {
                    let inner = node.text().trim().to_uppercase();
                    let tag =
                        self.short_prefix(&format!("-{}", inner.to_lowercase().replace(" ", "-")));
                    result.push_str(&format!("{}  *{}*\n\n", inner, tag))
                }
                _ => continue,
            }
        }

        return result;
    }

    fn parse(document: &'a Document) -> Self {
        let title = document
            .find(Class("page-header__title"))
            .next()
            .unwrap()
            .text();

        let categories = document
            .find(Class("page-header__categories-links"))
            .into_selection()
            .children()
            .iter()
            .filter(|node| node.name() == Some("a"))
            .map(|node| node.text())
            .collect::<Vec<String>>();

        let meta_text = document
            .find(Attr("id", "mw-content-text").child(Name("div")))
            .next()
            .unwrap()
            .find(Name("p"))
            .next()
            .unwrap()
            .text();

        let meta = meta_text
            .split('·')
            .map(|s| s.trim())
            .map(|s| s.split('\u{a0}').collect())
            .collect::<Vec<Vec<&str>>>();

        // dbg!(meta.clone());

        let mut entry = WikiEntry {
            n: 1,
            nodes: vec![],
            version: String::new(),
            url: "https://vim.fandom.com/wiki/VimTip1".to_owned(),
            title,
            categories,
            author: None,
            created: 0,
            complexity: String::new(),
        };

        for (i, node) in document
            .find(Attr("id", "mw-content-text"))
            .next()
            .unwrap()
            .children()
            .enumerate()
        {
            entry.nodes.push(node);
        }

        for m in meta {
            match m[0] {
                "version" => {
                    entry.version = m[1].to_owned();
                    continue;
                }
                "created" => {
                    entry.created = m[1].parse().unwrap_or(0);
                    continue;
                }
                "complexity" => {
                    entry.complexity = m[1].to_owned();
                    continue;
                }
                "author" => {
                    entry.author = Some(m[1].to_owned());
                    continue;
                }

                _ => continue,
            }
        }
        return entry;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = reqwest::get("https://vim.fandom.com/wiki/VimTip1").await?;
    // println!("{:#?}", resp);

    // let text = resp.text().await?;

    // let document = Document::from(text.as_str());
    let document = Document::from(include_str!("../searching.html"));
    let entry = WikiEntry::parse(&document);
    let result = entry.to_vim_help();
    let mut file = File::create(entry.file_name())?;
    file.write_all(&result.into_bytes())?;
    // println!("{:?}", result);
    // println!("{:?}", entry);

    Ok(())
}
