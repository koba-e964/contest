#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

a, b = map(int, readline().split())

def f(x):
    s = 0
    for i in range(x // 4 * 4, x + 1):
        s ^= i
    return s

print(f(a - 1) ^ f(b))
