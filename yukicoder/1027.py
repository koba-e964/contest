#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

d1, d2 = map(int, readline().split())

if d1 > d2:
    print(0)
elif d1 == d2:
    print(4)
elif d1 * 2 > d2:
    print(8)
elif d1 * 2 == d2:
    print(4)
else:
    print(0)
