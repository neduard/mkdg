import argparse
import datetime
import pathlib
import re
import shutil

import jinja2

title_re = re.compile(r'<h1>(.+)</h1>\n')
link_re = re.compile(r'<a href="(.+)">')
# date is optional and is of the form YYYY-MM-DD
post_name_re = re.compile(r'((\d{4})-(\d{2})-(\d{2})-)?.+')

class Post:
    def __init__(self, path):
        parsed_name = post_name_re.match(path.name)
        self.name = parsed_name.group(0)
        if parsed_name.group(1):
            self.date = datetime.date(*map(int, parsed_name.groups()[1:4]))
        else:
            self.date = None
        # Read file.
        self.lines = list(open(path, "r"))
        self.body = ''.join(self.lines)
        # regex ftw.
        self.title = title_re.match(self.lines[0]).group(1)
        self.links = link_re.findall(self.body)

        self.backlinks = []


def parse_weblog(top_path):
    post_paths = top_path.glob('*.html')

    weblog = { path.name : Post(path) for path in sorted(post_paths) }

    # Create backlinks.
    for name, post in weblog.items():
        for link in post.links:
            if not link.startswith('http'):
                weblog[link].backlinks.append(post)

    return weblog

def render_posts(weblog, env, out_path):
    out_path.mkdir(parents=True, exist_ok=True)

    for name, post in weblog.items():
        template = env.get_template('post.html')
        with open(out_path / name, 'w') as f:
            f.write(template.render(post=post))

    return weblog


def parse_args(args):
    parser = argparse.ArgumentParser(description='Simple weblog generator (with backlinks support!)')
    parser.add_argument('website_path', type=pathlib.Path)
    parser.add_argument('--output_dir', type=pathlib.Path, default='dist/')
    return parser.parse_args()


def main(args=None):
    args = parse_args(args)
    weblog = parse_weblog(args.website_path)
    env = jinja2.Environment(loader=jinja2.FileSystemLoader(args.website_path / 'templates'))

    if args.output_dir.exists():
        shutil.rmtree(args.output_dir)
        pass
    render_posts(weblog, env,  args.output_dir)

    # render index.html (imports base and overwrites content)
    template = env.get_template('page-list.html')
    with open(args.output_dir / 'page-list.html', 'w') as f:
        f.write(template.render(posts=weblog))

    # Copy images/ folder
    #TODO: this can include compression/optimization of the png files
    shutil.copytree(args.website_path / 'images', args.output_dir / 'images')

    for name, post in weblog.items():
        print(f'{name} links={post.links} '
              f'backlinks={[l.name for l in post.backlinks]} '
              f'title="{post.title}" '
              f'date="{post.date}"')

    # Useful for testing
    return weblog


if __name__ == '__main__':
    main()

