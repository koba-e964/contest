#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

t = [0, 0, 0, 1]
while t[len(t) - 4:] != [1, 0, 0, 0]:
    s = sum(t[len(t) - 4:]) % 17
    t.append(s)

p = len(t) - 3

q = int(readline())
for _ in range(q):
    n = int(readline())
    print(t[(n - 1) % p])
