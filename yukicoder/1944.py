#!/usr/bin/env python3

n, x, y = map(int, input().split())
r = list(map(int, input().split()))
r.sort()
t = r[0]
s = sum(r) - t
d = x * x + y * y
lo = max(0, t - 2 * s)
if lo * lo <= d <= (2 * s + t) * (2 * s + t):
    print('Yes')
else:
    print('No')
