#!/usr/bin/env python3

x, y, z = map(int, input().split())
s0, t0 = input().split()
s1, t1 = input().split()
t0 = int(t0)
t1 = int(t1)

d = min(min(x, y), z)

ans = x + y + z

if s0 == s1:
    ans = min(ans, abs(t0 - t1))

opp = {'A': x, 'B': y, 'C': z}
u0 = opp[s0] + 1 - t0
u1 = opp[s1] + 1 - t1

ans = min(ans, t0 + t1 - 1)
ans = min(ans, u0 + u1 - 1)

ans = min(ans, t0 + u1 + d - 1)
ans = min(ans, t1 + u0 + d - 1)

print(ans)
