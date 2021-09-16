#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

t = int(input())
for _ in range(t):
    a, b, c, d = input().split()
    print('{} {} {} {}'.format(a, b, int(c) + 1, d))
