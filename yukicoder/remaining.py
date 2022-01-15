#!/usr/bin/env python3
"""Prints yukicoder-related statistics (ratio of unsolved problems)
"""

import json
import requests

SOLVED_ENDPOINT = 'https://yukicoder.me/api/v1/solved/name/{}'
ALL_ENDPOINT = 'https://yukicoder.me/api/v1/problems'
USERNAME = 'koba-e964'

def main():
    """main function
    """
    resp = requests.get(ALL_ENDPOINT)
    problems = json.loads(resp.text)
    whole = sum(1 for _ in filter(lambda entry: entry['ProblemType'] == 0, problems))

    resp = requests.get(SOLVED_ENDPOINT.format(USERNAME))
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
    print(f'Unsolved: {whole - sl_count} / {whole} ({ratio:.3f} %)')
    print()
    print('remaining:')
    level_sum = 0.0
    solved_sum = 0.0

    for k in keys:
        solved_k = solved_map[k]
        all_k = levels[k]
        ratio = 100 * (1.0 - solved_k / all_k)
        print(f'Level {k}: {all_k - solved_k} / {all_k} ({ratio:.3f} %)')
        level_sum += all_k * k
        solved_sum += solved_k * k

    print()
    ratio = 100 * (1.0 - solved_sum / level_sum)
    print(f'Star {level_sum - solved_sum} / {level_sum} ({ratio:.3f}%)')

if __name__ == '__main__':
    main()
