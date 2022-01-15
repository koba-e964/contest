#!/usr/bin/env pypy3
"""
Returns the language-specific AC ranking and the AC deltas of participants
adjacent to you.
"""

import sys
import subprocess
import gzip
import json

ENDPOINT = 'https://kenkoooo.com/atcoder/resources/lang.json'
ENDPOINT_SUM = 'https://kenkoooo.com/atcoder/resources/sums.json'
LANGUAGE = 'Rust'
USERNAME = 'kobae964'


def fetch_ranking_lang():
    """Fetches language-wise rankings from ENDPOINT.
    """
    cmd = ["curl", "-s", "-H", "accept-encoding: gzip", ENDPOINT]
    output = subprocess.check_output(cmd)
    dec = gzip.decompress(output)
    dat = json.loads(dec)
    filt = []
    for entry in dat:
        if entry['language'] == LANGUAGE:
            filt.append(entry)
    return filt

def fetch_ranking(url):
    """Fetches #solved rankings from ENDPOINT_SUM.
    """
    cmd = ["curl", "-s", "-H", "accept-encoding: gzip", url]
    output = subprocess.check_output(cmd)
    dec = gzip.decompress(output)
    dat = json.loads(dec)
    return dat


def sort_ranking_by(rnk, key):
    """Sorts rnk by key.
    This function mutates rnk.
    """
    rnk.sort(key=lambda e: -e[key])
    return rnk


def display_result(rnk, username, key):
    ind = -1
    for i in range(len(rnk)):
        if rnk[i]['user_id'] == username:
            ind = i
            break
    if ind < 0:
        print(f'Error: user_id = {username} not found', file=sys.stderr)
        sys.exit(1)
    my_count = rnk[ind][key]
    seen = [0] + list(range(max(0, ind - 1), min(len(rnk), ind + 2)))
    seen = sorted(list(set(seen)))
    print(f'{"rank"}\t{"#"}\t{"delta"}\t{"name"}')
    for i in seen:
        print(f"{i + 1}\t{int(rnk[i][key])}\t{int(rnk[i][key] - my_count)}\t{rnk[i]['user_id']}")


def main():
    """main function
    """
    rnk = fetch_ranking_lang()
    sort_ranking_by(rnk, 'count')
    display_result(rnk, USERNAME, 'count')
    print()
    rnk = fetch_ranking(ENDPOINT_SUM)
    sort_ranking_by(rnk, 'point_sum')
    display_result(rnk, USERNAME, 'point_sum')


if __name__ == '__main__':
    main()
