#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

t = int(readline())
mo = 998244353
for _ in range(t):
    a, b, c = map(int, readline().split())
    r = (a ** 2 + b ** 2 + c ** 2) % mo
    r = r * pow(c, -2, mo) % mo
    print(r)
