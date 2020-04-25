#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

q = int(readline())


def f(n, i, j):
    layer = min(i, j, n - 1 - i, n - 1 - j)
    # editorial
    ans = n * n - (n - layer * 2) * (n - layer * 2) + 2 * (n - layer * 2 - 1)
    dist = (n - layer - 1 - i) + (n - layer - 1 - j)
    if i <= j:
        ans -= dist
    else:
        ans += dist
    return ans


for _ in range(q):
    n, i, j = map(int, readline().split())
    print(f(n, i, j))
