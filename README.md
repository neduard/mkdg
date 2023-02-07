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

1. Setup a plain http server: `python3 -m http.server --directory dist/`

2. Run:
```
$ python3 weblog.py demo/
```

3. Check out http://localhost:8000/

4. Hack away.
