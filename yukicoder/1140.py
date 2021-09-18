#!/usr/bin/env python3

import sys
import math
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

def eratosthenes(n):
    a = [True] * n
    a[0] = False
    a[1] = False
    for i in range(2, math.ceil(math.sqrt(n - 1))):
        if not a[i]:
            continue
        for j in range(2, (n - 1) // i + 1):
            a[i * j] = False
    return a


t = int(readline())
pr = eratosthenes(5000001)
for _ in range(t):
    a, p = map(int, readline().split())
    if pr[p]:
        print(0 if a % p == 0 else 1)
    else:
        print(-1)
