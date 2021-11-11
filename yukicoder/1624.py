#!/usr/bin/env python3

import math

a = float(input())
a = int(math.floor(1000 * a + 0.5))

def gcd(x, y):
    while y != 0:
        r = x % y
        x = y
        y = r
    return x


g = gcd(a, 1000)
p = a // g
q = 1000 // g

kind = 'A'
if p % 2 == 0:
    kind = 'B'
if q % 2 == 0:
    kind = 'C'

ans = p + q - 2 + (p + q) // 2 + abs(p - q) // 2
print('{} {}'.format(kind, ans))
