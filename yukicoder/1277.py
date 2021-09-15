#!/usr/bin/env pypy3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

s = 0.0
n = 1000
for a in range(1, n):
    for b in range(1, n):
        for c in range(1, n):
            s += 1 / a / b / c / (a + b + c - 1)

print(s)
zeta3 = 1.20205690315959428539973
print(s / zeta3)
