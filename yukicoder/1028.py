#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

def f(occ, x):
    tot = 0
    for (i, j) in occ:
        tot += max(abs(i - x), j)
    return tot


n = int(readline())
occ = [[] for _ in range(n)]
for i in range(n):
    aa = list(map(int, readline().split()))
    for j in range(n):
        occ[aa[j] - 1].append((i, j))

tot = 0
for i in range(n):
    lo = 0
    hi = n - 1
    while hi - lo > 2:
        mid1 = (hi + lo) // 2
        mid2 = mid1 + 1
        val1 = f(occ[i], mid1)
        val2 = f(occ[i], mid2)
        if val1 < val2:
            hi = mid2
        else:
            lo = mid1
    mi = 10000000
    for j in range(lo, hi + 1):
        mi = min(mi, f(occ[i], j))
    tot += mi

print(tot)
