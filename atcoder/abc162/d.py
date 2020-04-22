#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

MOD = 10 ** 9 + 7

n = int(readline())
s = list(readline().rstrip())

r = s.count(ord('R'))
g = s.count(ord('G'))
b = s.count(ord('B'))

count = r * g * b
for i in range(n):
    for j in range(1, min(i + 1, n - i)):
        if s[i - j] != s[i] and s[i] != s[i + j] and s[i - j] != s[i + j]:
            count -= 1

print(count)
