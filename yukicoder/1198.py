#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n = int(readline())
if n % 4 == 2 or n == 1 or n == 4:
    print(-1)
else:
    print(1)
