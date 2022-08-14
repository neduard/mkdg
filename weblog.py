import pathlib
import argparse
import mistune
import mistune.directives

class LinkAgreggator(mistune.HTMLRenderer):
    def __init__(self, page):
        super().__init__(escape=True)
        self.page = page

    def link(self, link, text=None, title=None):
        self.page.links.append(link)
        return super().link(link, text, title)


class Metadata(mistune.directives.Directive):
    def __init__(self, page):
        self.page = page

    def parse(self, block, m, state):
        self.page.meta = dict(self.parse_options(m))

    def __call__(self, md):
        self.register_directive(md, 'meta')


class Page:
    def __init__(self):
        self.body = None
        self.links = []
        self.backlinks = []
        self.meta = None
        self.path = None


def parse_weblog(top_path):
    weblog = {}

    for page_path in pathlib.Path(top_path).glob('**/*.md'):
        page = '/' + str(page_path.relative_to(top_path)).replace('.md', '.html')
        print(f'Processing {page}')
        weblog[page] = Page()
        weblog[page].path = page_path

        # Create a fresh parser for each page.
        link_agreggator = LinkAgreggator(weblog[page])
        metadata_directive = Metadata(weblog[page])
        md = mistune.create_markdown(renderer=link_agreggator, plugins=[metadata_directive])

        with open(page_path, 'r', encoding='utf-8') as input_file:
            weblog[page].body = md.parse(input_file.read())

    for p, v in weblog.items():
        for link in v.links:
            if not link.startswith('http'):
                weblog[link].backlinks.append(p)

    return weblog


def parse_args(args):
    parser = argparse.ArgumentParser(description='Simple weblog generator (with backlinks support!)')
    parser.add_argument('website_path', type=pathlib.Path, default='.')
    return parser.parse_args()

def main(args=None):
    args = parse_args(args)
    weblog = parse_weblog(args.website_path)
    # TODO further processing

    for page_name, page in weblog.items():
        print(f'{page_name} links={page.links} backlinks={page.backlinks} metadata={page.meta}')

    # Useful for testing
    return weblog


if __name__ == '__main__':
    main()

