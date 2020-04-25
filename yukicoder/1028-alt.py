#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

# editorial
n = int(readline())
occ = [[] for _ in range(n)]
for i in range(n):
    aa = list(map(int, readline().split()))
    for j in range(n):
        occ[aa[j] - 1].append((i, j))

tot = 0
for i in range(n):
    a = [0] * n
    b = [0] * (n + 1)
    for (x, y) in occ[i]:
        a[0] += max(x, y)
        if x - y >= 1:
            b[1] -= 1
            b[x - y + 1] += 1
        if x + y + 1 < n:
            b[x + y + 1] += 1
    for i in range(n - 1):
        b[i + 1] += b[i]
    for i in range(n):
        a[i] += b[i]
    for i in range(n - 1):
        a[i + 1] += a[i]
    mi = n * n
    for i in range(n):
        mi = min(mi, a[i])
    tot += mi

print(tot)
