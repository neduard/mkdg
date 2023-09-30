# Simple Digital Garden Generator

Create a static-site digital garden in <500 LoC:

```
wc -l src/*
  120 src/main.rs
  240 src/parser.rs
  360 total
```

This constraint achives a number of things:

1. It keeps things simple and easy to understand
2. Hence, it encourages [customisation and adoption](https://akkartik.name/freewheeling/)
3. It encourages more [creative solutions](https://en.wikipedia.org/wiki/Creative_limitation)
   that may not be perfect, but are still good most of the time

> Perfection is achieved, not when there is nothing more to add, but when there is nothing left to take away.
>
> ~Antoine de Saint-Exupéry

# Setup

1. [Install Rust](https://www.rust-lang.org/tools/install)

2. `cargo run -- --website-path demo` will compile the site to `dist/` and run
   a small server to view it locally

3. Open http://localhost:8000/

4. Hack away and repeat from step 2

5. Publish the created `dist/` folder to your hosting of choice


# Notes

The demo [base.html](demo/templates/base.html) is actually from the
[Hakyll](https://jaspervdj.be/hakyll/) example site.  Feel free to
write your own.  I used it because I was familiar with it.

# Features and simplifying assumptions

A summary of the feature list can also be found in the
[about](demo/about.md) [page](http://localhost:8000/about.html).

Here is a more detailed breakdown:

1. Support both HTML and Markdown
    * Markdown is first "compiled" into HTML
    * Plain HTML allows maximum flexibility
2. All pages are in a fixed directory structure.
    * this removes the need to recurse into directories
    * it also means that each page can be identified by *just* it's name and not the full path
3. Links are detected using a regex `<a href="(.+)?">`
    * Allows us to easily filter local links by checking if a link is prefixed with `http`
    * Regular pages + backlinks essentially mimic tags (but with the extra ability to add content)
    * Building the backlink table allows us to check for any dangling ones
4. Similar to links, the title is selected by a regex `<h1>(.+?)</h1>`
5. Distinguish between dated posts and regular pages purely by name:
    * a dated post is any page with a `YYYY-MM-DD-` prefix (eg. `2023-09-30-my-post.md`)
    * the two are still separated when listing the files in the folder
    * allows to distinguish one-off wirtings vs ongoing updating of a topic
6. Include a word counter to have an idea of how big is the website
