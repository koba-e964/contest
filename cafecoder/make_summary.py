#!/usr/bin/env python3
"""Make a summary from statuses.json.
"""

import json
import os


def main():
    """main function
    """
    script_dir = os.path.dirname(os.path.abspath(__file__))
    jsonfile = os.path.join(script_dir, 'statuses.json')
    tofile = os.path.join(script_dir, 'summary.md')

    with open(jsonfile, encoding='utf-8') as filep:
        data = json.load(filep)
    solved = 0
    whole = 0
    for contest in data:
        for value in contest['statuses']:
            if value['solved']:
                solved += 1
            whole += 1
    with open(tofile, mode='w', encoding='utf-8') as filep:
        print("| Remaining | All | Ratio |\n"
              "|----|----|----|\n"
              f"| {whole-solved} | {whole} | {(whole-solved)/whole*100:.4}% |",
              file=filep)

main()
