#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

a, b = map(int, readline().split())
if a == b:
    print(2 * a - 1)
else:
    print(2 * min(a, b))
