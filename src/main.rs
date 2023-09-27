use clap::Parser;
use parser::Page;
use rouille::Response;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use minijinja::{context, AutoEscape, Environment, Source};

mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    website_path: String,

    #[arg(short, long, default_value = "dist/")]
    output_dir: String,
}

fn render_website(pages: &Vec<Page>, env: &Environment, output_dir: &PathBuf) {
    fs::create_dir_all(&output_dir).unwrap();

    // Create pages.
    let template = env.get_template("page.html").unwrap();
    for page in pages {
        let file = fs::File::create(output_dir.join(&page.name)).unwrap();
        template.render_to_write(context! {page}, file).unwrap();
    }

    // Create page list.
    let template = env.get_template("page-list.html").unwrap();
    let file = fs::File::create(output_dir.join("page-list.html")).unwrap();
    template.render_to_write(context! { pages, }, file).unwrap();

    // Create index.
    let template = env.get_template("index.html").unwrap();
    let file = fs::File::create(output_dir.join("index.html")).unwrap();
    template
        .render_to_write(
            context! { word_count => pages.iter().fold(0, |acc, p| acc + p.word_count()) },
            file,
        )
        .unwrap();
}

fn process_images<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    // TODO: this can include compression/optimization of the png files
    fs_extra::copy_items(&vec![from], to, &fs_extra::dir::CopyOptions::default()).unwrap();
}

fn main() {
    let args = Args::parse();
    let website_path = Path::new(&args.website_path);
    let pages = parser::load_pages(&website_path);

    let mut env = Environment::new();
    env.set_auto_escape_callback(|_| AutoEscape::None);
    env.set_source(Source::from_path(website_path.join("templates")));

    let output_dir = PathBuf::from_str(&args.output_dir).unwrap();
    if output_dir.exists() {
        panic!(
            "{} already exists.  Please remove it first.",
            output_dir.display()
        )
    }

    render_website(&pages, &env, &output_dir);

    fs::copy(
        website_path.join("sp-0.9.0.css"),
        &output_dir.join("default.css"),
    )
    .unwrap();

    process_images(website_path.join("images"), &output_dir);

    for page in pages {
        println!(
            "{} links={:?} backlinks={:?} title=\"{}\" words={}",
            page.name,
            page.links,
            page.backlinks,
            page.title,
            page.word_count()
        );
    }

    static ADDRESS: &str = "127.0.0.1:8000";
    println!("Starting demo server on http://{ADDRESS}");
    rouille::start_server(ADDRESS, move |request| {
        let request_path = format!("{}/{}", output_dir.to_str().unwrap(), request.url());
        println!("GET {}", &request_path);

        // Check if the requested file exists
        if Path::new(&request_path).is_file() {
            let content_type = if request_path.ends_with("css") {
                "text/css"
            } else if request_path.ends_with("png") {
                "img/png"
            } else {
                "text/html"
            };
            let file = std::fs::File::open(request_path).unwrap();
            Response::from_file(content_type, file)
        } else {
            // Serve the default index.html file if the requested file is not found
            let file = std::fs::File::open(format!("{}/index.html", output_dir.to_str().unwrap()))
                .unwrap();
            Response::from_file("text/html", file)
        }
    });
}
