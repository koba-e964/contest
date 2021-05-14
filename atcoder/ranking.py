#!/usr/bin/env python3
"""
Returns the language-specific AC ranking and the AC deltas of participants
adjacent to you.
"""

import sys
import subprocess
import gzip
import json

ENDPOINT = 'https://kenkoooo.com/atcoder/resources/lang.json'
LANGUAGE = 'Rust'
USERNAME = 'kobae964'


def fetch_ranking():
    cmd = ["curl", "-s", "-H", "accept-encoding: gzip", ENDPOINT]
    output = subprocess.check_output(cmd)
    dec = gzip.decompress(output)
    dat = json.loads(dec)
    filt = []
    for entry in dat:
        if entry['language'] == LANGUAGE:
            filt.append(entry)
    return filt


def sort_ranking(rnk):
    rnk.sort(key=lambda e: -e['count'])
    return rnk


def display_result(rnk, username):
    ind = -1
    for i in range(len(rnk)):
        if rnk[i]['user_id'] == username:
            ind = i
            break
    if ind < 0:
        print("Error: user_id = {} not found".format(username), file=sys.stderr)
        sys.exit(1)
    my_count = rnk[ind]['count']
    seen = [0] + list(range(max(0, ind - 1), min(len(rnk), ind + 2)))
    seen = sorted(list(set(seen)))
    print("{}\t{}\t{}\t{}".format("rank", "#", "delta", "name"))
    for i in seen:
        print("{}\t{}\t{}\t{}".format(i + 1, rnk[i]['count'], rnk[i]['count'] - my_count, rnk[i]['user_id']))


def main():
    rnk = fetch_ranking()
    sort_ranking(rnk)
    display_result(rnk, USERNAME)


if __name__ == '__main__':
    main()
