#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

s = input()
l = len(s) // 2
t = s[:l]
print('YES' if t + t == s else 'NO')
