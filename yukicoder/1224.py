#!/usr/bin/env python3

import sys
import math
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

a, b = map(int, readline().split())
g = math.gcd(a, b)
a //= g
b //= g
while True:
    x = math.gcd(b, 10)
    if x == 1:
        break
    b //= x

print('Yes' if b != 1 else 'No')
