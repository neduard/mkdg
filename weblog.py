import pathlib
import argparse
import shutil

import graphviz
import mistune
import mistune.directives
import jinja2

class LinkAgreggator(mistune.HTMLRenderer):
    def __init__(self, post):
        super().__init__(escape=True)
        self.post = post

    def link(self, link, text=None, title=None):
        self.post.links.append(link)
        return super().link(link, text, title)


class Metadata(mistune.directives.Directive):
    def __init__(self, post):
        self.post = post

    def parse(self, block, m, state):
        self.post.meta = dict(self.parse_options(m))

    def __call__(self, md):
        self.register_directive(md, 'meta')


class Post:
    def __init__(self, name):
        self.name = name
        self.body = None
        self.links = []
        self.backlinks = []
        self.meta = None

    @property
    def title(self):
        return self.meta['title']

    # TODO: implement property for date.


def parse_weblog(top_path):
    weblog = {}
    posts_dir = top_path / 'posts'

    for post_path in posts_dir.glob('*.md'):
        name = post_path.name.replace('.md', '.html')
        post = Post(name)

        # Create a fresh parser for each post.
        link_agreggator = LinkAgreggator(post)
        metadata_directive = Metadata(post)
        md = mistune.create_markdown(renderer=link_agreggator, plugins=[metadata_directive])

        with open(post_path, 'r', encoding='utf-8') as input_file:
            post.body = md.parse(input_file.read())

        # Add to weblog
        weblog[name] = post

    for p, v in weblog.items():
        for link in v.links:
            if not link.startswith('http'):
                weblog[link].backlinks.append(v)

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
    parser.add_argument('website_path', type=pathlib.Path, default='.')
    return parser.parse_args()


def main(args=None):
    args = parse_args(args)
    weblog = parse_weblog(args.website_path)
    env = jinja2.Environment(loader=jinja2.FileSystemLoader(args.website_path))

    output_dir = args.website_path / 'dist'
    if output_dir.exists():
        shutil.rmtree(output_dir)
        pass
    render_posts(weblog, env,  output_dir / 'posts')

    # render index.html (imports base and overwrites content)
    template = env.get_template('index.html')
    with open(output_dir / 'index.html', 'w') as f:
        f.write(template.render(posts=weblog))

    # render Knowledge Graph
    dot = graphviz.Digraph()
    for name, post in weblog.items():
        dot.node(name, label=name, URL=f"posts/{name}")

    for name, post in weblog.items():
        edges = [(name, p2.name) for p2 in post.backlinks]
        dot.edges(edges)
    svg_str = str(dot.pipe('svg'), 'utf-8')
    svg_str_trimmed = svg_str[svg_str.index('<svg'):].strip()
    template = env.get_template('/knowledge_graph.html')
    with open(output_dir / 'knowledge_graph.html', 'w') as f:
        f.write(template.render(svg=svg_str_trimmed))

    # Copy css/ folder
    shutil.copytree(args.website_path / 'css', output_dir / 'css')

    # Copy images/ folder
    #TODO: this can include compression/optimization of the png files
    shutil.copytree(args.website_path / 'images', output_dir / 'images')

    for name, post in weblog.items():
        print(f'{name} links={post.links} backlinks={[l.name for l in post.backlinks]} metadata={post.meta}')

    # Useful for testing
    return weblog


if __name__ == '__main__':
    main()

