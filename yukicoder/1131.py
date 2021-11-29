#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n = int(readline())
a = list(map(int, readline().split()))

s = sum(a)
for i in range(n):
    print(((100 + a[i]) * n - s) // (2 * n))
