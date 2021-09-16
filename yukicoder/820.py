#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n, k = map(int, input().split())
print(pow(2, n - k) if n >= k else 0)
