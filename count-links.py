#!/usr/bin/env python3

import argparse
import bs4
import glob
import multiprocessing
import os

def count_links_in_page(path: str):
    with open(path) as html_file:
        html_doc = html_file.read()

    soup = bs4.BeautifulSoup(html_doc, 'html.parser')
    return sum(1 for link in soup.find_all('a') if link.get('href'))

def main(dataset_path: str):
    assert os.path.exists(dataset_path)

    with multiprocessing.Pool() as pool:
        result = list(pool.imap_unordered(count_links_in_page, glob.glob(f'{dataset_path}/**/*.html', recursive=True)))
    links_count = sum(result)
    pages_count = len(result)

    print(f"In {pages_count} found {links_count} links")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Computes number of links in dataset HTML files")
    parser.add_argument('dataset_path', help='Path to dataset')
    args = parser.parse_args()
    main(args.dataset_path)
