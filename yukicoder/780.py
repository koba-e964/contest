#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

a, b = map(int, input().split())
d = abs(a + 1 - b)
print('YES' if a + 1 <= b else 'NO')
print(d)
