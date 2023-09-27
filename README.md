# Simple Digital Garden Generator

Create a static-site digital garden in <500 LoC.

```
wc -l src/*
  120 src/main.rs
  240 src/parser.rs
  360 total
```

# Setup

1. [Install Rust](https://www.rust-lang.org/tools/install)

2. `cargo run -- --website-path demo` will compile the site to `dist/` and run
   a small server to view it locally

3. Open http://localhost:8000/

4. Hack away

5. Publish the created `dist/` folder to your hosting of choice


# Notes

The demo [base.html](demo/templates/base.html) is actually from the
[Hakyll](https://jaspervdj.be/hakyll/) example site.

# Features and simplifying assumptions

A summary of the feature list can also be found in the
[about](http://localhost:8000/about.html) page.

Here is a more detailed breakdown:

1. Support both HTML and Markdown
  * Markdown is first "compiled" into HTML
  * Plain HTML allows maximum flexibility
1. All pages are in a fixed directory structure.
  * this removes the need to recurse into directories
  * it also means that each page can be identified by *just* it's name and not the full path
1. Links are detected using `<a href="(.+)?">`
  * Allows us to easily filter local links by checking if a link is prefixed with `http`
  * Regular pages + backlinks essentially mimic tags (but with the extra ability to add content)
  * Building the backlink table allows us to check for any dangling ones
1. Title of a page is of the form `<h1>Title With Spaces</h1>`
  * Might be a bit clunky, but we can easily search for it using a regex
1. Distinguish between dated posts (YYYY-MM-DD- prefix) and regular pages
purely from name
  * the two are still separated when listing the files in the folder
  * allows to distinguish one-off wirtings vs ongoing updating of a topic
1. Include a small word counter to have an idea of how big the website is.
