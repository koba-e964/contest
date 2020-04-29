#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n = int(readline())
front = []
back = []
for _ in range(n):
    t, s = readline().rstrip().split()
    if t == b'0':
        front.append(s)
    else:
        back.append(s)


back.reverse()
back.extend(front)
print(b''.join(back).decode('ascii'))
