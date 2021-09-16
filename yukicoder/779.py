#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

a, b, c = map(int, input().split())
print('Yes' if (a, b, c) >= (1989, 1, 8) and (a, b, c) <= (2019, 4, 30) else 'No')
