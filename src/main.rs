use clap::{Parser};
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{PathBuf, Path};
use chrono::NaiveDate;

use minijinja::{Environment, Source, context};

#[derive(Debug)]
struct Post {
    name: String,
    date: Option<NaiveDate>,
    lines: Vec<String>,
    body: String,
    title: String,
    links: Vec<String>,
    backlinks: Vec<String>,
}

impl Post {
    fn new(path: &PathBuf) -> Self {
        let title_re = Regex::new(r"<h1>(.+)</h1>").unwrap();
        // Use ? character to specify non-greedy matching (minimal)
        let link_re = Regex::new(r#"<a href="(.+?)">"#).unwrap();
        let post_name_re = Regex::new(r"((\d{4})-(\d{2})-(\d{2})-)?.+").unwrap();

        let name = path.file_name().unwrap().to_str().unwrap().to_owned();
        println!("name: {name:?}");
        let parsed_name = post_name_re.captures(&name).unwrap();
        let date = if let Some(date_match) = parsed_name.get(1) {
            let year: i32 = parsed_name.get(2).unwrap().as_str().parse().unwrap();
            let month: u32 = parsed_name.get(3).unwrap().as_str().parse().unwrap();
            let day: u32 = parsed_name.get(4).unwrap().as_str().parse().unwrap();
            Some(NaiveDate::from_ymd(year, month, day))
        } else {
            None
        };

        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
        let body = lines.join("");
        println!("line[0] {:?} for file {:?}", lines[0], path.to_str());
        let title = title_re.captures(&lines[0]).unwrap()[1].to_string();
        let links = link_re
            .captures_iter(&body)
            .map(|cap| cap[1].to_string())
            .collect::<Vec<_>>();

        Post {
            name,
            date,
            lines,
            body,
            title,
            links,
            backlinks: Vec::new(),
        }
    }
}

fn parse_site(top_path: &std::path::Path) -> std::collections::HashMap<String, Post> {
    let post_paths = top_path
        .read_dir()
        .expect("Expected a directory")
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file() && path.extension().expect("Unable to get extension") == "html")
        .collect::<Vec<_>>();
    
    let mut weblog = post_paths
        .into_iter()
        .map(|path| {
            let post = Post::new(&path);
            (post.name.clone(), post)
        })
        .collect::<std::collections::HashMap<_, _>>();
    
    let _weblog_clone = weblog.iter();
    
    // Create backlinks.
    let backlink_pairs: Vec<(String, Vec<String>)> = weblog
        .iter()
        .map(|(name, post)| 
            (name.clone(),
             post
                .links 
                .clone()
                .iter()
                .filter(|link| !link.starts_with("http"))
                .map(|x| (*x).clone() )
                .collect()))
        .collect();
         
    for (name, links) in backlink_pairs {
        println!("Name {name}");
        for link in links {
            println!("  Link: \"{}\"", &link);
            weblog.get_mut(&link).unwrap().backlinks.push(name.clone());
        }
    }
    weblog
}

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    website_path: String,
    
    #[arg(short, long, default_value = "dist/")]
    output_dir: String,
}

fn main() {
    let args = Args::parse();
    let top_path = Path::new(&args.website_path);
    let weblog = parse_site(&top_path);
    
    let templates_path = Path::new(&args.website_path).join("templates");
    
    let mut env = Environment::new();
    
    env.set_source(Source::from_path(templates_path));
    let template = env.get_template("base.html").unwrap();
    println!("{}", template.render(context! {name => "Eduard" }).unwrap());
}