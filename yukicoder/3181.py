#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

_, gx, gy = map(int, readline().split())
print(gx * 3, gy * 3)
