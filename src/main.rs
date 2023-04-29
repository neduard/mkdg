use clap::Parser;
use parser::Page;
use rouille::Response;
use std::fs;
use std::path::{PathBuf, Path};
use std::str::FromStr;

use minijinja::{Environment, Source, context, AutoEscape};

mod parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    website_path: String,
    
    #[arg(short, long, default_value = "dist/")]
    output_dir: String,
}

fn render_website(
    pages: &Vec<Page>,
    env: &Environment,
    output_dir: &PathBuf,
) {
    fs::create_dir_all(&output_dir).unwrap();

    // Create pages.
    let template = env.get_template("page.html").unwrap();
    for page in pages {
        let file = fs::File::create(output_dir.join(&page.name)).unwrap();
        template.render_to_write(context!{page}, file).unwrap();
    }
    
    // Create page list.
    let template = env.get_template("page-list.html").unwrap();
    let file = fs::File::create(output_dir.join("page-list.html")).unwrap();
    template.render_to_write(
        context! { pages, },
        file).unwrap();
    
    // Create index.
    let template = env.get_template("index.html").unwrap();
    let file = fs::File::create(output_dir.join("index.html")).unwrap();
    template.render_to_write(
        context! { word_count => pages.iter().fold(0, |acc, p| acc + p.word_count()) },
        file).unwrap();
}

fn main() {
    let args = Args::parse();
    let website_path= Path::new(&args.website_path);
    let pages = parser::load_pages(&website_path);
    let templates_path = Path::new(&args.website_path.clone()).join("templates");
    
    let mut env = Environment::new();
    env.set_auto_escape_callback(|_| AutoEscape::None);
    env.set_source(Source::from_path(templates_path));
    
    let output_dir = PathBuf::from_str(&args.output_dir).unwrap();
    render_website(&pages, &env, &output_dir);
    
    
      
    for page in pages {
        println!(
            "{} links={:?} backlinks={:?} title=\"{}\" words={}",
            page.name, page.links, page.backlinks, page.title, page.word_count()
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