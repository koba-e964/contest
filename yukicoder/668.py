#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

s = readline().rstrip()
l = len(s)
k = int(s[:3])
k = (k + 5) // 10
if k >= 100:
    print('1.0*10^{}'.format(l))
else:
    print('{}.{}*10^{}'.format(k // 10, k % 10, l - 1))
