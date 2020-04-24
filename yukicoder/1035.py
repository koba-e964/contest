#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n, m = map(int, readline().split())

mod = 10**9 + 7

sum = 0
cur = 1
for i in range(0, m + 1):
    tmp = pow(i, n, mod)
    if (m + i) % 2 != 0:
        tmp = mod - tmp
    sum += tmp * cur
    cur *= (m - i) * pow(i + 1, mod - 2, mod)
    cur %= mod

print(sum % mod)
