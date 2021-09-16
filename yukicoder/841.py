#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

s, t = input().split()
if s[0] == 'S':
    if t[0] == 'S':
        print('8/33')
    else:
        print('8/32')
else:
    print('8/31')
