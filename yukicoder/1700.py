#!/usr/bin/env python3

def sq(n):
    hi = 1000000001
    lo = 0
    while hi - lo > 1:
        mid = (hi + lo) // 2
        if mid * mid <= n:
            lo = mid
        else:
            hi = mid
    return lo

t = int(input())

for _ in range(t):
    n = int(input())
    print(int(sq(n)))
