#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

x = int(readline())
print(x % 21)
