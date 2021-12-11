#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

t, b = map(int, readline().split())
for _ in range(t):
    n = int(readline())
    s = ''
    while n != 0:
        r = n % (-b)
        s += chr(48 + r)
        n = (n - r) // b
    if s == '':
        s = '0'
    print(''.join(reversed(s)))
