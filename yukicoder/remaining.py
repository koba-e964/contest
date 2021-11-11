#!/usr/bin/env python3

import sys
import gzip
import json
import requests

SOLVED_ENDPOINT = 'https://yukicoder.me/api/v1/solved/name/{}'
ALL_ENDPOINT = 'https://yukicoder.me/api/v1/problems'
USERNAME = 'koba-e964'

def main():
    resp = requests.get(ALL_ENDPOINT)
    problems = json.loads(resp.text)
    whole = sum(1 for _ in filter(lambda entry: entry['ProblemType'] == 0, problems))

    resp = requests.get(SOLVED_ENDPOINT.format(USERNAME))
    solved = json.loads(resp.text)

    levels = {}
    for p in problems:
        if p['ProblemType'] != 0:
            continue
        lev = p['Level']
        if lev not in levels:
            levels[lev] = 0
        levels[lev] += 1
    keys = [k for k in levels]
    keys.sort()
    sl = {k: 0 for k in keys}
    sl_count = 0
    for p in solved:
        if p['ProblemType'] != 0:
            continue
        lev = p['Level']
        sl[lev] += 1
        sl_count += 1
    print('Solved: {}, All: {}'.format(sl_count, whole))
    print('Unsolved: {} / {} ({:.3f} %)'
          .format(whole - sl_count, whole, 100 * (1.0 - sl_count / whole)))
    print()
    print('remaining:')
    level_sum = 0.0
    solved_sum = 0.0

    for k in keys:
        a = sl[k]
        b = levels[k]
        print('Level {}: {} / {} ({:.3f} %)'
              .format(k, b - a, b, 100 * (1.0 - a / b)))
        level_sum += b * k
        solved_sum += a * k

    print()
    print('Star {} / {} ({:.3f}%)'
          .format(level_sum - solved_sum, level_sum,
                  100 * (1.0 - solved_sum / level_sum)))

if __name__ == '__main__':
    main()
