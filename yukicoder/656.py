#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

s = input()
a = 0
for c in s:
    if c == '0':
        a += 10
    else:
        a += ord(c) - 48
print(a)
