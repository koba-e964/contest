#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

x = int(readline())

q = (x + 10) // 11
tot = 0
if 11 * q - x >= 5:
    tot -= 1

tot += q * 2
print(tot)
