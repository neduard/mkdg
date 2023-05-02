# Python Digital Garden Generator

A simple digital garden generator with backlink support.

Objectives:

1. Simplicity.  This is more like a script, not a framework.

# Setup

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `cargo run -- `

```
$ python3 weblog.py demo/
```

3. Check out http://localhost:8000/

4. Hack away.


# Notes

At the moment, the CSS theme and base.html are actually from the [Hakyll](https://jaspervdj.be/hakyll/) example site.

# Design

A few simplifying assumptions are made:

1. Use HTML instead of Markdown.
  * Markdown can be used as long as it is 'compiled' to HTML (via whatever means)
  * HTML allows maximum flexibility at the (IMO) low cost of being more verbose
2. All pages are in a fixed directory structure.
  * this removes the need to recurse into directories
  * it also means that each page can be identified by *just* it's name and not the full path
3. Distinguish between dated posts (YYYY-MM-DD- prefix) and regular pages purely from name
  * the two are still separated when listing the files in the folder
  * allows to distinguish one-off wirtings vs ongoing updating of a topic
4. Links are detected using `<a href="(.+)?">`
  * Allows us to easily filter local links by checking if a link is prefixed with `http`
  * Regular pages + backlinks essentially mimic tags (but with the extra ability to add content)
5. Title is of the form `<h1>Title With Spaces</h1>`
  * Again, might be a bit clunky, but we can easily search for it using a regex
