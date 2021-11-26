#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n = int(readline())
a = list(map(int, readline().split()))

ans = 0
b = [0] * (n + 2)
for i in range(n - 1, -1, -1):
    if a[i] == 1:
        if i < n - 1 and a[i + 1] == 0:
            b[i] = b[i + 2] + 2
        else:
            b[i] = 1
    else:
        b[i] = 0
    b[i] = min(b[i], n - i - 1)
    ans += b[i]

print(ans)
