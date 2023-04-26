use clap::{Parser};
use regex::Regex;
use rouille::Response;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{PathBuf, Path};
use std::str::FromStr;

use minijinja::{Environment, Source, context, AutoEscape};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Post {
    name: String,
    date_y_m_d: Option<(u32, u32, u32)>,
    lines: Vec<String>,
    body: String,
    title: String,
    links: Vec<String>,
    backlinks: Vec<(String, String)>,
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
        let date_y_m_d = if let Some(_) = parsed_name.get(1) {
            let year: u32 = parsed_name.get(2).unwrap().as_str().parse().unwrap();
            let month: u32 = parsed_name.get(3).unwrap().as_str().parse().unwrap();
            let day: u32 = parsed_name.get(4).unwrap().as_str().parse().unwrap();
            Some((year, month, day))
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
            date_y_m_d,
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
    let backlink_pairs: Vec<(String, String, Vec<String>)> = weblog
        .iter()
        .map(|(name, post)| 
            (name.clone(),
             post.title.clone(),
             post
                .links 
                .clone()
                .iter()
                .filter(|link| !link.starts_with("http"))
                .map(|x| (*x).clone() )
                .collect()))
        .collect();
         
    for (name, title, links) in backlink_pairs {
        println!("Name {name}");
        for link in links {
            println!("  Link: \"{}\"", &link);
            weblog.get_mut(&link).unwrap().backlinks.push((name.clone(), title.clone()));
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

fn render_posts(
    weblog: &std::collections::HashMap<String, Post>,
    env: &Environment,
    out_path: &PathBuf,
) {
    fs::create_dir_all(&out_path).unwrap();

    for post in weblog.values() {
        let template = env.get_template("post.html").unwrap();
        let file = fs::File::create(out_path.join(&post.name)).unwrap();
        template.render_to_write(context!{post => post}, file).unwrap();
    }
}

fn main() {
    let args = Args::parse();
    let website_path= Path::new(&args.website_path);
    let weblog = parse_site(&website_path);
    println!("{:?}", weblog.values()) ;
    let templates_path = Path::new(&args.website_path.clone()).join("templates");
    
    let mut env = Environment::new();
    env.set_auto_escape_callback(|_| AutoEscape::None);
    
    env.set_source(Source::from_path(templates_path));
    let template = env.get_template("page-list.html").unwrap();
    
    let output_dir = PathBuf::from_str(&args.output_dir).unwrap();
    render_posts(&weblog, &env, &output_dir);
    
    let file = fs::File::create(output_dir.join("page-list.html")).unwrap();
    template.render_to_write(
        context!
            {posts => weblog.iter().collect::<Vec<_>>()},
        file).unwrap();
        
    for post in weblog.values() {
        println!(
            "{} links={:?} backlinks={:?} title=\"{}\"",
            post.name, post.links, post.backlinks, post.title
        );
    }
        
    rouille::start_server("127.0.0.1:8080", move |request| {
        let request_path = format!("{}/{}", output_dir.to_str().unwrap(), request.url());

        // Check if the requested file exists
        if Path::new(&request_path).is_file() {
            let file = std::fs::File::open(request_path).unwrap();
            
            Response::from_file("text/html", file)
        } else {
            let file = std::fs::File::open(
                format!("{}/index.html", output_dir.to_str().unwrap())).unwrap();
            // Serve the default index.html file if the requested file is not found
            
            Response::from_file("text/html", file)
        }
    });
}