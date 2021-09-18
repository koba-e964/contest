#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n, d = map(int, input().split())
input()
v = sum(map(int, input().split()))

print((d + v - 1) // v)
