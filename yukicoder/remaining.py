#!/usr/bin/env python3
"""Prints yukicoder-related statistics (ratio of unsolved problems)
"""

import json
import os
import sys
import requests

SOLVED_ENDPOINT = 'https://yukicoder.me/api/v1/solved/name/{}'
ALL_ENDPOINT = 'https://yukicoder.me/api/v1/problems'

def percentage(nominator, denominator):
    """Get the percentage (integer, rounded to nearest)
    If the value rounded to nearest is 100 and nominator < denominator,
    this function returns 99.
    """
    perc = (nominator * 200 + denominator) // (2 * denominator)
    if perc == 100 and nominator < denominator:
        perc = 99
    return perc


def main():
    """main function
    """
    emit_md = False
    if 'EMIT_MD' in os.environ and os.environ['EMIT_MD'] == '1':
        emit_md = True

    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} USERNAME", file=sys.stderr)
        sys.exit(1)
    username = sys.argv[1]
    resp = requests.get(ALL_ENDPOINT)
    problems = json.loads(resp.text)
    whole = sum(1 for _ in filter(lambda entry: entry['ProblemType'] == 0, problems))

    resp = requests.get(SOLVED_ENDPOINT.format(username))
    solved = json.loads(resp.text)

    levels = {}
    for prob in problems:
        if prob['ProblemType'] != 0:
            continue
        lev = prob['Level']
        if lev not in levels:
            levels[lev] = 0
        levels[lev] += 1
    keys = list(levels)
    keys.sort()
    solved_map = {k: 0 for k in keys}
    sl_count = 0
    for prob in solved:
        if prob['ProblemType'] != 0:
            continue
        lev = prob['Level']
        solved_map[lev] += 1
        sl_count += 1
    print(f'Solved: {sl_count}, All: {whole}')
    ratio = 100 * (1.0 - sl_count / whole)
    if emit_md:
        print()
        print('| |unsolved|whole|ratio|progress|')
        print('|----|----|----|----|----|')
        prog = percentage(sl_count, whole)
        print(f'|**All**| {whole - sl_count} | {whole} | {ratio:.3f} %| ![{prog}%](https://progress-bar.dev/{prog}?title=All) |')
    else:
        print(f'Unsolved: {whole - sl_count} / {whole} ({ratio:.3f} %)')
        print()
        print('remaining:')
    level_sum = 0.0
    solved_sum = 0.0

    for k in keys:
        solved_k = solved_map[k]
        all_k = levels[k]
        ratio = 100 * (1.0 - solved_k / all_k)
        if emit_md:
            prog = percentage(solved_k, all_k)
            print(f'|**Level {k}**| {all_k - solved_k} | {all_k} | {ratio:.3f} %| ![{prog}%](https://progress-bar.dev/{prog}?title=Level+{k})|')
        else:
            print(f'Level {k}: {all_k - solved_k} / {all_k} ({ratio:.3f} %)')
        level_sum += all_k * k
        solved_sum += solved_k * k

    ratio = 100 * (1.0 - solved_sum / level_sum)
    if emit_md:
        prog = percentage(int(10 * solved_sum), int(10 * level_sum))
        print(f'|**Star**|{level_sum - solved_sum} | {level_sum} |{ratio:.3f}%| ![{prog}%](https://progress-bar.dev/{prog}?title=Star) |')
    else:
        print()
        print(f'Star {level_sum - solved_sum} / {level_sum} ({ratio:.3f}%)')

if __name__ == '__main__':
    main()
