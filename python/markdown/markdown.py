"""
Module to parse markdown text to render it in HTML
"""

import re


def parse(markdown):
    """Parse a markdown string"""
    for operation in [
            parse_header,
            parse_bold,
            parse_italic,
            parse_list_items,
            parse_unordered_list,
            parse_paragraphs]:
        markdown = operation(markdown)
    # tests don't like newline character
    return markdown.replace('\n',"")


def parse_header(markdown: str) -> str:
    """
    If a line begins with "#", then determine its header level
    and output the line with the appropriate header tag.
    ex: "### My Line" -> "<h3> My Line </h3>"
    """

    # MULTILINE lets us match headers throughout the markdown string.
    # DOTALL allows us to separate the match into the #s, the title, and everything that remains.
    header_match = re.match(r'^\s*(#+)(\s+[^\n]*)(.*)',markdown, flags=re.MULTILINE|re.DOTALL)
    if not header_match:
        return markdown

    level = len(header_match.group(1))
    if level > 6:
        return markdown

    # Needed to left strip whitespace to pass whitespace sensitive test
    title = header_match.group(2).lstrip()
    remaining = header_match.group(3)
    print(header_match.groups())
    return f"<h{level}>{title}</h{level}>{remaining}"

def parse_bold(markdown: str) -> str:
    """__bold thing__ -> <strong>bold thing</strong>"""
    return re.sub(r'__(.*?)__', r"<strong>\1</strong>", markdown)

def parse_italic(markdown: str) -> str:
    """_italic thing_ -> <em>italic thing</em>"""
    return re.sub(r'_(.*?)_', r"<em>\1</em>", markdown)

def parse_list_items(markdown: str) -> str:
    """Use beginning * to denote list item"""
    return re.sub(r'^\* (.*)',r'<li>\1</li>', markdown, flags=re.MULTILINE)

def parse_unordered_list(markdown: str) -> str:
    """
    Envelop grouped list items with <ul> tag.
    Must run after parse_list_items.
    """
    return re.sub(r'<li>(.*)</li>',r'<ul><li>\1</li></ul>',markdown,flags=re.DOTALL)

def parse_paragraphs(markdown: str) -> str:
    """Wrap any line not already tagged with <p> tag"""
    return re.sub(r'^(?!<[hlu])(.*?$)', r"<p>\1</p>", markdown, flags=re.MULTILINE)
