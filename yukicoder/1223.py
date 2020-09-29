#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n, k, t = map(int, readline().split())
print('Yes' if abs(n) <= k * t else 'No')
