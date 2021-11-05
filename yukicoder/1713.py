#!/usr/bin/env python3

a, b = map(int, input().split())
c = a | b
ans = 1
for i in range(1, c + 1):
    ans *= i
print(ans)
