import pathlib
import argparse
import mistune

class LinkAgreggator(mistune.InlineParser):
    def __init__(self, renderer, hard_wrap=True):
        super().__init__(renderer, hard_wrap)
        self.links = []

    def parse_link(self, m, state):
        super().parse_link(m, state)

        tokens = state.tokens
        # TODO: understand / document how inline parsing works and
        # how it produces tokens.
        for token in tokens:
            if token['type'] == 'image':
                # Ignore image links
                return
            if token['type'] == 'link':
                url = token['attrs']['url']
                if url not in self.links:
                    self.links.append(url)
                return

class Page:
    def __init__(self):
        self.body = None
        self.links = None
        self.backlinks = []
        self.path = None


def parse_weblog(top_path):
    weblog = {}

    for page_path in pathlib.Path(top_path).glob('**/*.md'):
        page = '/' + str(page_path.relative_to(top_path)).replace('.md', '.html')
        print(f'Processing {page}')
        weblog[page] = Page()
        weblog[page].path = page_path

        # Create a fresh parser for each page.
        link_agreggator = LinkAgreggator(mistune.HTMLRenderer(escape=True))
        md = mistune.Markdown(renderer=link_agreggator.renderer,
                              inline=link_agreggator)

        with open(page_path, 'r', encoding='utf-8') as input_file:
            weblog[page].body = md.parse(input_file.read())
        weblog[page].links = link_agreggator.links

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
        print(f'{page_name} links={page.links} backlinks={page.backlinks}')

    # Useful for testing
    return weblog


if __name__ == '__main__':
    main()

