#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

s = readline().rstrip()
a = 0
for c in s:
    if c != 48:
        a += 1

print(a - 1)
