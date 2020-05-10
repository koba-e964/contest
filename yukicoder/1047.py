#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

a, b = map(int, readline().split())

if b == 0:
    print(1)
    exit()

if a == -1:
    print(2)
    exit()

print(-1)
