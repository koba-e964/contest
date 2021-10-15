#!/usr/bin/env python3

import sys
import math
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

n = int(readline())
x = list(map(int, readline().split()))
m = int(readline()) - 1

s = 0
sq = 0

for a in x:
    s += a
    sq += a * a

v = sq * n - s * s
if v == 0:
    print('50')
else:
    d = x[m] * n - s
    ans = d / math.sqrt(v) * 10 + 50
    if ans < 0:
        ans = math.ceil(ans)
    else:
        ans = math.floor(ans)
    print(int(ans))
