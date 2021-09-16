#!/usr/bin/env python3

import sys
import math
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n = int(input())
for _ in range(n):
    x = int(input())
    print(x * x * 3 * math.sqrt(3) / 4)
