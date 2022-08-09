import markdown
import pathlib
import pdb
import argparse
import collections

class LinkAgreggator(markdown.treeprocessors.Treeprocessor):
    def run(self, root):
        self.links = []
        self.agreggate_links(root)

    def agreggate_links(self, element):
        for child in element:
            if child.tag == 'a':
                self.links.append(child.attrib['href'])
            self.agreggate_links(child)

class Page:
    def __init__(self):
        self.body = None
        self.links = None
        self.backlinks = []


def parse_weblog(top_path):
    weblog = {}
    # Initial conversion
    for page_path in pathlib.Path(top_path).glob('**/*.md'):
        page = '/' + str(page_path.relative_to(top_path)).replace('.md', '.html')
        print(f'Processing {page}')
        weblog[page] = Page()
        parser = markdown.Markdown()
        link_agreggator = LinkAgreggator(parser)
        parser.treeprocessors.register(link_agreggator, 'links', 1)
        with open(page_path, "r", encoding="utf-8") as input_file:
            weblog[page].body = parser.convert(input_file.read())
        weblog[page].links = link_agreggator.links

    for p, v in weblog.items():
        print(p)
        print(v.links)
        for link in v.links:
            if not link.startswith('http'):
                weblog[link].backlinks.append(page)

    return weblog

def filter_local_links(links):
    return [link for link in links if not link.startswith('http')]

def parse_args(args):
    parser = argparse.ArgumentParser(description='Simple weblog generator (with backlinks support!)')
    parser.add_argument('website_path', type=pathlib.Path, default='.')
    return parser.parse_args()

def main(args=None):
    args = parse_args(args)
    weblog = parse_weblog(args.website_path)
    # TODO further processing

    # Useful for testing
    return weblog

if __name__ == '__main__':
    main()

