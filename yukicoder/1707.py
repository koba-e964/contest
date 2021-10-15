#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

t = int(readline())

def calc(a):
    n = len(a) // 2
    f = [[] for _ in range(n + 1)]
    for i in range(2 * n):
        f[a[i]].append(i)
    for i in range(1, n + 1):
        if len(f[i]) != 2:
            return False
    if a[0] != 1 or a[2 * n - 1] != n:
        return False
    if n == 1:
        return True
    if all(a[i] == i % n + 1 for i in range(2 * n)):
        return True
    x = f[1][1]
    if x == n and f[n][0] == 1:
        x = 0
    b = [0] * (2 * n)
    y = x // 2
    for i in range(y + 1):
        b[i] = i + 1
    for i in range(y):
        b[y + 1 + i] = y - i
    for i in range(n - y):
        b[2 * y + 1 + i] = n - i
    for i in range(n - y - 1):
        b[2 * y + 1 + n - y + i] = y + 2 + i
    return a == b

for _ in range(t):
    n = int(readline())
    a = list(map(int, readline().split()))
    print('Yes' if calc(a) else 'No')
