#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n = int(input())
n = max(n, 0)
print(n // 100)
