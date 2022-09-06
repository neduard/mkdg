# Python Digital Garden Generator

A simple digital garden generator with backlink support.

Objectives:

1. Simplicity.  This is a script, not a framework.
2. Hackability.  Users are encouraged to read the source code and ajust to their own preferences.  PRs are welcome as long as they don't conflict with objective no. 1

# Installation

```
python3 -m venv venv
./venv/bin/pip3 install -r requirements.txt
```

# Running

0. activate venv environment `source ./venv/bin/activate`

1. Setup a plain http server: `python3 -m http.server --directory test/dist/`

2. Run:
```
$ python3 weblog.py test/
p1.html links=[] backlinks=['p2.html', 'p3.html'] metadata={'title': 'This is page 1', 'moreinfo': 'some more info'}
p2.html links=['p1.html'] backlinks=['p3.html'] metadata={'title': '"This is page 2"'}
p3.html links=['p1.html', 'p2.html'] backlinks=[] metadata={'title': '"This is page 3"'}
```

3. Check out http://0.0.0.0:8000/

4. Hack away.


# Notes

At the moment, the CSS theme and base.html are actually from the [Hakyll](https://jaspervdj.be/hakyll/) example site.
