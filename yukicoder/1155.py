#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n = int(readline())
if n == 1:
    print('Shiitakerando')
elif n == 2:
    print('Otsukakokusaibijutsukan')
else:
    print('Spring-8')
