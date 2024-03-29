use regex::Regex;
use serde::Serialize;

type Title = String;
type Name = String;

#[derive(Debug, Serialize)]
pub struct Page {
    pub name: Name,
    pub date_formatted: Option<String>,
    pub body: String,
    pub title: Title,
    pub links: Vec<String>,
    pub backlinks: Vec<(Name, Title)>,
}

impl Page {
    fn from_string(name: String, body: String) -> Page {
        let title_re = Regex::new(r"<h1>(.+?)</h1>").unwrap();
        // Use ? character to specify non-greedy matching (minimal)
        // Also filter out page headings if present.
        let link_re = Regex::new(r#"<a href="([^#]+?)(#.*)?".*>"#).unwrap();
        let date_re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})-.+").unwrap();

        let title = title_re
            .captures(&body)
            .expect(&format!("Unable to find title in {}", &body))[1]
            .to_owned();

        let links: Vec<String> = link_re
            .captures_iter(&body)
            .map(|captures| captures[1].to_string())
            // Get only unique links.
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let date = if let Some(captures) = date_re.captures(&name) {
            let get_number = |index, element| {
                captures
                    .get(index)
                    .expect(&format!("Unable to parse {element}"))
                    .as_str()
                    .parse()
                    .expect("Unable to parse number")
            };
            Some(
                chrono::NaiveDate::from_ymd_opt(
                    get_number(1, "year"),
                    get_number(2, "month").try_into().unwrap(),
                    get_number(3, "day").try_into().unwrap(),
                )
                .unwrap(),
            )
        } else {
            None
        };

        Page {
            name,
            date_formatted: date.map(|d| d.format("%B %d, %Y").to_string()),
            body,
            title,
            links,
            backlinks: Vec::new(),
        }
    }

    fn from_html(path: &std::path::Path) -> Page {
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .to_owned()
            .expect(&format!("Unable to extract file name for {:?}", path))
            .to_owned();
        let body = std::fs::read_to_string(path)
            .expect(&format!("Unable to read {}", path.to_str().unwrap()));
        Page::from_string(name, body)
    }

    fn from_md(path: &std::path::Path) -> Page {
        let name: String = path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .expect(&format!("Unable to extract file name for {:?}", path))
            .replace(".md", ".html");
        let body_md: String = std::fs::read_to_string(path)
            .expect(&format!("Unable to read {}", path.to_str().unwrap()));
        let body = markdown::to_html_with_options(
            &body_md,
            &markdown::Options {
                compile: markdown::CompileOptions {
                    // As a static website generator, we only parse files
                    // written by the user.  There's no untrusted inputs.
                    allow_dangerous_html: true,
                    allow_dangerous_protocol: true,
                    ..markdown::CompileOptions::gfm()
                },
                ..markdown::Options::gfm()
            },
        )
        .expect("Unable to parse markdown");
        Page::from_string(name, body)
    }

    pub fn word_count(self: &Self) -> usize {
        let html_tag = Regex::new(r"<.+?>").unwrap();
        html_tag
            .replace_all(&self.body, " ")
            .split_whitespace()
            .filter(|str| str.len() >= 2)
            .count()
    }
}

pub fn load_pages(input_dir: &std::path::Path) -> Vec<Page> {
    let mut pages_map = input_dir
        .read_dir()
        .expect("Expected a directory")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .filter_map(|path| {
            let extention = path.extension()?;
            if extention == "html" {
                let page = Page::from_html(&path);
                Some((page.name.clone(), page))
            } else if extention == "md" {
                let page = Page::from_md(&path);
                Some((page.name.clone(), page))
            } else {
                None
            }
        })
        .collect::<std::collections::HashMap<_, _>>();

    // Create backlinks.
    // First, select all the local links.
    let page_links: Vec<(Name, Title, Vec<Name>)> = pages_map
        .iter()
        .map(|(name, page)| {
            (
                // Create the result tuple.
                name.clone(),
                page.title.clone(),
                // Go through the links and select the ones that are local.
                // i.e. DON'T start with http and end in .html
                page.links
                    .iter()
                    .filter_map(|link| {
                        if !link.starts_with("http") && link.ends_with(".html") {
                            Some((*link).clone())
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        })
        .collect();

    // Now we build the backlinks.
    for (name, title, links) in page_links {
        for link in links {
            pages_map
                .get_mut(&link) // Get the page with the corresponding link.
                .expect(&format!(
                    "{}: Unable to find link {}.  Maybe unexistent page?",
                    &name, &link
                ))
                .backlinks
                .push((name.clone(), title.clone())); // Add a backlink to that page.
        }
    }

    // Sort pages by name.
    let mut pages: Vec<Page> = pages_map.into_values().collect();
    pages.sort_by(|p1, p2| p1.name.cmp(&p2.name));
    pages
}

#[cfg(test)]
mod tests {
    use super::load_pages;

    fn create_test_pages(dir: &std::path::Path) {
        std::fs::write(
            &dir.join("test_page_1.html"),
            b"<h1>First Test page</h1>
                <p>Some text with a <a href=\"test_page_2.html\">link</a></p>
                <h1>Another title just to be confusing</h1>
                <p>Some more text with another <a href=\"https://www.google.com\">link</a></p>
                <a href=\"2022-02-22-test_page_3.html\">Another link</a>",
        )
        .expect("Unable to write page 1");

        std::fs::write(
            &dir.join("test_page_2.html"),
            b"<h1>Second Test Page</h1>
                <p>Let's create a cycle <a href=\"test_page_1.html\">link</a></p>
                <p>Let's create a cycle again <a href=\"test_page_1.html\">another link</a></p>",
        )
        .expect("Unable to write page 2");

        std::fs::write(
            &dir.join("2022-02-22-test_page_3.html"),
            b"<h1>Need a title element else it breaks</h1><br>
                <p>Let's create a self cycle <a href=\"2022-02-22-test_page_3.html\">link</a></p>
                <p>Let's create a another self cycle
                <a href=\"2022-02-22-test_page_3.html\">another link</a></p>",
        )
        .expect("Unable to write page 3");

        std::fs::write(
            &dir.join("2022-02-23-test_page_4.html"),
            b"<h1>Title</h1><br>
                <p>Can't have enough <a href=\"2022-02-22-test_page_3.html#header\">links</a></p>",
        )
        .expect("Unable to write page 3");

        std::fs::write(
            &dir.join("word_count.html"),
            b"<h1>Title</h1><br>
                <p>Can't have <b>enough</b> words with <a href=\"2022-02-22-test_page_3.html\">
                links</a></p>.  Also . s in gl e && do u bl e    l e tt er s don't count. !!!",
        )
        .expect("Unable to write page 3");
    }

    #[test]
    fn new_page() {
        let tmpdir = tempfile::tempdir().expect("Unable to create tempdir");
        create_test_pages(&tmpdir.path());
        let pages = load_pages(&tmpdir.path());

        println!("Pages should be sorted alphabetically");
        assert_eq!(pages[0].name, "2022-02-22-test_page_3.html");
        assert_eq!(pages[1].name, "2022-02-23-test_page_4.html");
        assert_eq!(pages[1].word_count(), 5);
        assert_eq!(pages[2].name, "test_page_1.html");
        assert_eq!(pages[3].name, "test_page_2.html");
        assert_eq!(pages[4].name, "word_count.html");
        assert_eq!(pages[4].word_count(), 18);

        println!("Section links are ignored");
        assert_eq!(pages[0].links, vec!["2022-02-22-test_page_3.html"]);
        assert_eq!(pages[1].links, vec!["2022-02-22-test_page_3.html"]);
    }
}
