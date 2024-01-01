#!/usr/bin/env python3

import sys
import math
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

a = int(readline())
b = int(readline())
c = int(readline())
for bd in range(1, b + c - a):
    for ec in range(1, b + c - a - bd):
        bc = a + bd + ec
        if bc <= abs(b - c):
            continue
        cosb = (b * b + bc * bc - c * c) / (2 * b * bc)
        cosc = (-b * b + bc * bc + c * c) / (2 * c * bc)
        ad = math.sqrt(b * b + bd * bd - 2 * cosb * b * bd)
        ae = math.sqrt(c * c + ec * ec - 2 * cosc * c * ec)
        sinb = math.sqrt(1 - cosb * cosb)
        sinc = math.sqrt(1 - cosc * cosc)
        sinbad = sinb * bd / ad
        sineac = sinc * ec / ae
        if abs(sinbad - sineac) <= 1.0e-9:
            print('Yes')
            exit()
print('No')
