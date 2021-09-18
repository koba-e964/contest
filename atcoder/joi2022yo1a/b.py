#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

x = int(readline())
y = int(readline())
z = int(readline())
print(1 if z >= x + y else 0)
