#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

s = readline().rstrip()
l = len(s)
k = sum(i == 111 for i in s)

for i in s:
    print(k / l * 100)
    if i == 111:
        k -= 1
    l -= 1
