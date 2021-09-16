#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n, m = map(int, input().split())
a = 0
for _ in range(n):
    s = input()
    a += s.count('W')
print(a)
