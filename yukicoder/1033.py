#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n, _k = map(int, readline().split())
print(n / 2.0)
