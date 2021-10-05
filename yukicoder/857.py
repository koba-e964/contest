#!/usr/bin/env python3

x, y, z = map(int, input().split())

a = z
if x <= z:
    a -= 1
if y <= z:
    a -= 1
print(a)
