#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

s = input()
print(s.replace('I', '1').replace('l', '1').replace('O', '0').replace('o', '0'))
