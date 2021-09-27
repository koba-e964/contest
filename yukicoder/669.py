#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n, k = map(int, readline().split())
a = map(int, readline().split())

g = 0
for b in a:
    g ^= b % (k + 1)

print('YES' if g != 0 else 'NO')
