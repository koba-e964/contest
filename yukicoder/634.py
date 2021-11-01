#!/usr/bin/env python3

import math

n = int(input())

for x in range(5000):
    if x * (x + 1) == 2 * n:
        print(1)
        exit()

for x in range(5000):
    r = n - x * (x + 1) // 2
    if r >= 0:
        r = 8 * r + 1
        s = int(math.floor(math.sqrt(r)))
        if s * s == r:
            print(2)
            exit()

print(3)
