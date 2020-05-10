#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n, k = map(int, readline().split())
a = list(map(int, readline().split()))

a.sort()
a.reverse()

tot = a[0]
for i in range(1, k):
    tot += max(a[i], 0)
print(tot)

