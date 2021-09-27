#!/usr/bin/env python3

n, k = map(int, input().split())
if n % 2 == 0:
    print(min(n // 2, k + 1))
else:
    print(min(n, k + 1))
