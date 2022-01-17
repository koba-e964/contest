#!/usr/bin/env python3
"""Make a summary from statuses.json.
"""

import json
import os


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
    script_dir = os.path.dirname(os.path.abspath(__file__))
    jsonfile = os.path.join(script_dir, 'statuses.json')
    tofile = os.path.join(script_dir, 'summary.md')

    with open(jsonfile, encoding='utf-8') as filep:
        data = json.load(filep)
    solved = 0
    whole = 0
    for contest in data:
        for value in contest['statuses']:
            if value['ignored']:
                continue
            if value['solved']:
                solved += 1
            whole += 1
    with open(tofile, mode='w', encoding='utf-8') as filep:
        prog = percentage(solved, whole)
        print("| Remaining | All | Ratio | Progress |\n"
              "|----|----|----|----|\n"
              f"| {whole-solved} | {whole} | {(whole-solved)/whole*100:.2f}% | ![{prog}%](https://progress-bar.dev/{prog}?title=solved) |",
              file=filep)

main()
